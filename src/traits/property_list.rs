use std::collections::HashMap;
use crate::traits::lead_object::*;

pub struct PropertyList{
    children: Vec<LeadObject>,
    strings: HashMap<String, String>,
    floats: HashMap<String, f32>,
    ints: HashMap<String, i32>,
    bools: HashMap<String, bool>,
}

impl PropertyList{
    pub fn new() -> Self {
        PropertyList{
            children: Vec::new(),

            strings: HashMap::new(),
            floats: HashMap::new(),
            ints: HashMap::new(),
            bools: HashMap::new(),
        }
    }

    pub fn is_property_type(name: &str) -> bool {
        match name {
            "string" => true,
            "float" => true,
            "int" => true,
            "bool" => true,
            _ => false
        }
    }

    pub fn is_property_valid(name: &str, attrs: &HashMap<String, String>) -> bool {
        match name {
            "string" | "float" | "int" | "bool" => attrs.contains_key("name") && attrs.contains_key("value"),
            _ => false
        }
    }

    pub fn add_property(&mut self, p_type: &str, attrs: &HashMap<String, String>) {
        let key = attrs.get("name").unwrap().to_owned();
        let value = attrs.get("value").unwrap().to_owned();
        match p_type {
            "string" => self.set_string(key, value),
            "int" => {
                if let Ok(int_value) = value.parse::<i32>() {
                    self.set_int(key, int_value);
                }
            }
            "float" => {
                if let Ok(float_value) = value.parse::<f32>() {
                    self.set_float(key, float_value);
                }
            }
            "bool" => {
                match value.as_str() {
                    "true" => self.set_bool(key, true),
                    "false" => self.set_bool(key, false),
                    _ => panic!("Unable to parse {value} to a bool!")
                }
            }
            _ => {}
        }
    }

    pub fn add_child(&mut self, child: LeadObject) {
        self.children.push(child);
    }

    pub fn set_string(&mut self, k: String, v: String) {
        self.strings.insert(k, v);
    }

    pub fn set_float(&mut self, k: String, v: f32) {
        self.floats.insert(k, v);
    }
    
    pub fn set_int(&mut self, k: String, v: i32) {
        self.ints.insert(k, v);
    }

    pub fn set_bool(&mut self, k: String, v: bool) {
        self.bools.insert(k, v);
    }

    pub fn get_string(&self, k: &str, default: &str) -> String {
        self.strings.get(k).cloned().unwrap_or_else(|| default.to_string())
    }

    pub fn get_float(&self, k: &str, default: f32) -> f32 {
        self.floats.get(k).cloned().unwrap_or_else(|| default)
    }

    pub fn get_int(&self, k: &str, default: i32) -> i32 {
        self.ints.get(k).cloned().unwrap_or_else(|| default)
    }
    
    pub fn get_bool(&self, k: &str, default: bool) -> bool {
        self.bools.get(k).cloned().unwrap_or_else(|| default)
    }

}