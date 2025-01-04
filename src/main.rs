pub mod traits;
pub mod impls;
pub mod utils;

pub mod macros;
pub mod common;
pub mod factory;
pub mod parser;

use common::LeadObject;
use parser::Parser;
use crate::common::Vector2f;

fn main() {
    let parser = Parser::new();
    let main_scene_obj = match parser.parse_file("./scenes/temp.xml"){
        Ok(root_node) => root_node,
        Err(e) => panic!("Ran into error {:?}", e)
    };

    let main_scene  = match main_scene_obj {
        LeadObject::Scene(scene) => scene,
        _ => panic!("Couldnt find a scene!"),
    };

    let v1 = Vector2f::init_one(0f32);
    let v2 = Vector2f::init(1f32, 2f32);
    let v = v1 + v2;

    println!("{}", v.to_string());

    println!("{}", main_scene.to_string());
}
