use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::common::*;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file<P: AsRef<Path>>(&self, filename: P) -> Result<LeadObject, String> {
        let file = File::open(filename).map_err(|e| e.to_string())?;
        let mut reader = Reader::from_reader(BufReader::new(file));

        let mut buf = Vec::new();
        let mut root_node: Option<LeadObject> = None;
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if root_node.is_none() {
                        root_node = Some(self.traverse_node(&mut reader, e)?)
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("Error reading XML: {:?}", e)),
                _ => ()
            }
            buf.clear();
        }
        
        match root_node {
            Some(root) => Ok(root),
            None => Err("Error reading XML, no root found".to_string())
        }
    }

    pub fn traverse_node(
        &self,
        reader: &mut Reader<BufReader<File>>,
        start: &BytesStart,
    ) -> Result<LeadObject, String> {
        let mut node_type = String::from_utf8_lossy(start.name().into_inner()).into_owned();
        let attributes = self.get_attributes(start).unwrap();
        let mut prop_list: PropertyList = PropertyList::new();
        let mut children: Vec<LeadObject> = Vec::new();
        let mut buf: Vec<u8>= Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let child_name = String::from_utf8_lossy(e.name().into_inner()).into_owned();
                    if PropertyList::is_property_type(&child_name) {
                        let child_attrs = self.get_attributes(e).unwrap();
                        if PropertyList::is_property_valid(&child_name, &child_attrs) {
                            prop_list.add_property(&child_name, &child_attrs);
                        } else {
                            panic!("Non-valid property defined!");
                        }
                    } else {
                        let child = self.traverse_node(reader, e)?;
                        children.push(child);
                    }
                }
                // Since empty nodes can only be properties, we are only checking for that
                Ok(Event::Empty(ref e)) => {
                    let child_name = String::from_utf8_lossy(e.name().into_inner()).into_owned();
                    if PropertyList::is_property_type(&child_name) {
                        let child_attrs = self.get_attributes(e).unwrap();
                        if PropertyList::is_property_valid(&child_name, &child_attrs) {
                            prop_list.add_property(&child_name, &child_attrs);
                        } else {
                            panic!("Non-valid property defined!");
                        }
                    }
                }
                Ok(Event::End(ref e)) if e.name() == start.name() => break,
                Ok(Event::Eof) => panic!("Unexpected EOF while parsing file!"),
                Err(e) => return Err(format!("Error reading XML: {:?}", e)),
                _ => {}
            }
            buf.clear();
        }

        if attributes.contains_key("type") {
            node_type = attributes.get("type").unwrap().to_string();
        }

        let mut obj = create_lead_object(&node_type, prop_list);
        for child in children.into_iter() {
            obj.add_child(child);
        }

        return Ok(obj);
    }

    fn get_attributes(&self, start: &BytesStart) -> Result<HashMap<String, String>, String> {
        let mut attrs = HashMap::new();
        for attr in start.attributes() {
            let attr = attr.map_err(|e| e.to_string())?;
            let key = String::from_utf8_lossy(attr.key.into_inner()).to_string();
            let val = String::from_utf8_lossy(&attr.value).to_string();

            attrs.insert(key, val);
        }

        Ok(attrs)
    }
}