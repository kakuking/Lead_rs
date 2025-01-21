use crate::common::*;

pub struct Scene{
    shapes: Vec<Arc<dyn Shape>>,
    _accel: Option<Arc<dyn Aggregate>>,
}

// Constructor
fn create_scene(prop_list: PropertyList) -> LeadObject {
    let mut scene = Scene::new();
    scene.init(prop_list);
    LeadObject::Scene(Arc::new(scene))
}

// imp lead object
impl LeadObjectTrait for Scene {
    fn init(&mut self, _prop_list: PropertyList) { }

    fn activate(&mut self) {
        
    }

    fn add_child(&mut self, child: LeadObject) {
        match child {
            LeadObject::Shape(shape) => self.shapes.push(shape),
            _ => println!("Struct Scene does not take a child of class {}", child.to_string())
        };
    }

    fn to_string(&self) -> String {
        let mut shapes_part = String::new();
        for shape in self.shapes.iter() {
            shapes_part += &shape.to_string();
            shapes_part += "\n";
        };
        
        
        format!(
            "Scene[\n  shapes: {{\n{}\n  }}\n]",
            indent(&shapes_part, 4)
        )
    }
}

impl Scene{
    pub fn new() -> Self {
        Scene {
            shapes: Vec::new(),
            _accel: None
        }
    }

    pub fn get_camera(&self) -> String {
        String::from("Scene method")
    }

}

register_struct!("scene", create_scene);