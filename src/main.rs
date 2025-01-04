pub mod traits;
pub mod impls;
pub mod factory;
pub mod common;
pub mod parser;

// use common::PropertyList;
use common::LeadObject;
use parser::Parser;

fn main() {
    // let prop_list = PropertyList::new();
    // let main_scene_obj =  factory::create_lead_object("scene", prop_list);
    let parser = Parser::new();
    let main_scene_obj = match parser.parse_file("./scenes/temp.xml"){
        Ok(root_node) => root_node,
        Err(e) => panic!("Ran into error {:?}", e)
    };

    let main_scene  = match main_scene_obj {
        LeadObject::Scene(scene) => scene,
        _ => panic!("Couldnt find a scene!"),
    };

    println!("{}", main_scene.to_string());
}
