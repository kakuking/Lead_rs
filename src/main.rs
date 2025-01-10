pub mod traits;
pub mod impls;
pub mod utils;

pub mod macros;
pub mod common;
pub mod factory;
pub mod parser;

use common::{LeadObject, Point2f};
use parser::Parser;

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

    let temp = Point2f::init([1.0, 2.0]);
    let temp2 = Point2f::init([1.0, 1.0]);
    let temp3 = temp - temp2;
    println!("{}, {}", main_scene.to_string(), temp3.to_string());
}
