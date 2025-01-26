pub mod accel;
pub mod camera;
pub mod filter;
pub mod geometry;
pub mod integrator;
pub mod light;
pub mod material;
pub mod medium;
pub mod sampler;
pub mod scene;
pub mod shapes;
pub mod spectrum;
pub mod utils;

pub mod macros;
pub mod common;
pub mod factory;
pub mod parser;

use common::LeadObject;
use parser::Parser;

fn main() {
    let parser = Parser::new();
    let mut main_scene_obj = match parser.parse_file("./scenes/temp.xml"){
        Ok(root_node) => root_node,
        Err(e) => panic!("Ran into error {:?}", e)
    };

    main_scene_obj.activate();

    let main_scene  = match main_scene_obj {
        LeadObject::Scene(scene) => scene,
        _ => panic!("Couldnt find a scene!"),
    };

    println!("{}", main_scene.to_string());
}
