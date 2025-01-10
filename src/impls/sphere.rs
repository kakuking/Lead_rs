// use crate::common::*;

// pub struct Sphere {
//     radius: f32
// }

// fn create_sphere(prop_list: PropertyList) -> LeadObject {
//     let mut sphere = Sphere::new();
//     sphere.init(prop_list);
//     LeadObject::Shape(Box::new(sphere))
// }

// impl Shape for Sphere {
    
// }

// impl LeadObjectTrait for Sphere {
//     fn init(&mut self, prop_list: PropertyList) {
//         self.radius = prop_list.get_float("radius", 0.5f32);
//     }

//     fn add_child(&mut self, child: LeadObject) {
//         println!("Struct Sphere does not take a child of class {}", child.to_string());
//     }

//     fn to_string(&self) -> String {
//         format!("sphere: [\n  radius: {}\n]", self.radius)
//     }
// }

// impl Sphere{
//     pub fn new() -> Self {
//         Sphere{
//             radius: 0f32
//         }
//     }
// }

// register_struct!("sphere", create_sphere);