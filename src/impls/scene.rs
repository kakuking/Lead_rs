use crate::{common::*, utils::primitive::GeometricPrimitive};

pub struct Scene{
    shapes: Vec<Arc<dyn Shape>>,
    accel: Arc<dyn Aggregate>,
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
        let mut primitives: Vec<Arc<dyn Primitive>> = Vec::new();

        while let Some(cur_shape) = self.shapes.pop() {
            let prim: GeometricPrimitive = GeometricPrimitive::init_shape(cur_shape);
            primitives.push(Arc::new(prim));
        }

        if primitives.len() > 0 {
            let mut bvh: BVHAccel = BVHAccel::new();
            bvh.create(primitives, 120, SplitMethod::SAH);
            self.accel = Arc::new(bvh);
        }
    }

    fn add_child(&mut self, child: &mut LeadObject) {
        match child {
            LeadObject::Shape(shape) => self.shapes.push(shape.clone()),
            _ => println!("Struct Scene does not take a child of class {}", child.to_string())
        };
    }

    fn to_string(&self) -> String {
        let mut shapes_part = String::new();
        for prim in self.accel.primitives() {
            shapes_part += &prim.shape().unwrap().to_string();
            shapes_part += "\n";
        };
        
        format!(
            "Scene[\n  shapes: {{\n{}\n  }}\n]",
            indent(&shapes_part, 4)
        )
    }
}

impl SceneTrait for Scene {

}

impl Primitive for Scene {
    fn intersect(&self, ray: &Ray, its: &mut SurfaceInteraction) -> bool {
        self.accel.intersect(ray, its)
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.accel.intersect_p(ray)
    }

    fn compute_scattering_functions(&self, _its: &SurfaceInteraction, _mode: TransportMode, _allow_multiple_lobes: bool) {
        panic!("Not implemented yet!");
    }

    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>> {
        panic!("Not implemented yet!");
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        panic!("Not implemented yet!");
    }

    fn world_bound(&self) -> Bounds3f {
        panic!("Not implemented yet!");
    }

    fn shape(&self) -> Option<Arc<dyn Shape>> {
        None
    }
}

impl Scene{
    pub fn new() -> Self {
        Scene {
            shapes: Vec::new(),
            accel: Arc::new(BVHAccel::new())
        }
    }

    pub fn get_camera(&self) -> String {
        String::from("Scene method")
    }

}

register_struct!("scene", create_scene);