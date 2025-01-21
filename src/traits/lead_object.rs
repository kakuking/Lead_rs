use crate::common::*;


pub trait LeadObjectTrait {
    fn init(&mut self, prop_list: PropertyList);
    fn activate(&mut self);
    fn to_string(&self) -> String;
    fn add_child(&mut self, child: LeadObject);
}

pub enum LeadObject {
    Scene(Arc<dyn LeadObjectTrait>),
    Shape(Arc<dyn Shape>),
    Unknown(())
}

impl LeadObject {
    pub fn to_string(&self) -> &str {
        match self {
            LeadObject::Scene(_) => "scene",
            LeadObject::Shape(_) => "shape",
            LeadObject::Unknown(_) => "Unknown",
        }
    }

    pub fn add_child(&mut self, child: LeadObject) {
        match self {
            LeadObject::Scene(s) => Arc::get_mut(s).unwrap().add_child(child),
            LeadObject::Shape(s) => Arc::get_mut(s).unwrap().add_child(child),
            LeadObject::Unknown(_) => panic!("Cannot add child to unknown object!")
        };
    }
}