use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::common::PropertyList;
use crate::traits::lead_object::LeadObject;

type LeadObjCtor = fn(PropertyList) -> LeadObject;

// start a factory
lazy_static!{
    static ref REGISTERY: Mutex<HashMap<&'static str, LeadObjCtor>> = {
        Mutex::new(HashMap::new())
    };
}

// Register something to the factory
pub fn register_lead_object(name: &'static str, ctor: LeadObjCtor) {
    let mut registery = REGISTERY.lock().unwrap();
    registery.insert(name, ctor);
}

// Create a lead object
pub fn create_lead_object(name: &str, prop_list: PropertyList) -> LeadObject {
    let registery = REGISTERY.lock().unwrap();
    match registery.get(name) {
        Some(ctor) => ctor(prop_list),
        None => panic!("No constructor found for struct {name}")
    }
}