use crate::common::*;

pub struct Sphere {
    radius: f32,
    z_min: f32, z_max: f32,
    theta_min: f32, theta_max: f32, phi_max: f32,

    object_to_world: Transform, world_to_object: Transform,
    bounding_box: Bounds3f,
    reverse_orientation: bool, 
}

fn create_sphere(prop_list: PropertyList) -> LeadObject {
    let sphere = Sphere::new(prop_list);
    LeadObject::Shape(Box::new(sphere))
}

impl Shape for Sphere {
    fn object_to_world(&self) -> Transform { self.object_to_world.clone() }

    fn world_to_object(&self) -> Transform { self.world_to_object.clone() }

    fn reverse_orientation(&self) -> bool { self.reverse_orientation }

    fn transform_swaps_handedness(&self) -> bool { self.object_to_world.swaps_handedness() }

    fn get_object_bounds(&self) -> Bounds3f { self.bounding_box.clone() }

    fn get_world_bounds(&self) -> Bounds3f { &self.object_to_world * self.bounding_box.clone() }

    fn area(&self) -> f32 {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }

    fn pdf(&self, its: &Box<dyn Interaction>) -> f32 {
        let x = its.p().x();
        let y = its.p().y();
        let z = its.p().z();

        let theta = (z / self.radius).acos();
        if theta > self.theta_max || theta < self.theta_min {
            return 0f32;
        }

        let phi = y.signum() * (x / (x*x + y*y).sqrt()).acos();
        if phi > self.phi_max {
            return 0f32;
        }

        1f32 / self.area()
    }

    fn pdf_wi(&self, reference: &Box<dyn Interaction>, _wi: &Vector3f) -> f32 {
        if reference.p().z() > self.z_max || reference.p().z() < self.z_min {
            return 0f32;
        }

        let x = reference.p().x();
        let y = reference.p().y();
        let phi = y.signum() * (x / (x*x + y*y).sqrt()).acos();
        if phi > self.phi_max {
            return 0f32;
        }

        1f32 / self.area()
    }

    fn sample(&self, _reference: &Box<dyn Interaction>, u: &Point2f) -> Box<dyn Interaction> {
        self.sample_u(u)
    }

    fn sample_u(&self, _u: &Point2f) -> Box<dyn Interaction> {
        Box::new(SurfaceInteraction::new())
    }

    fn intersect(&self, _ray: &Ray, _t_hit: &mut f32, _its:  &mut SurfaceInteraction) -> bool {
        false
    }

    fn intersect_p(&self, _ray: &Ray) -> bool {
        false
    }
}

impl LeadObjectTrait for Sphere {
    fn init(&mut self, _prop_list: PropertyList) { }

    fn add_child(&mut self, child: LeadObject) {
        println!("Struct Sphere does not take a child of class {}", child.to_string());
    }

    fn to_string(&self) -> String {
        format!("sphere: [\n  radius: {}\n  z_min: {}\n  z_max: {}\n  phi_max: {}\n  bounding_box: \n{}\n  object_to_world: \n{}\n]", self.radius, self.z_min, self.z_max, self.phi_max, indent(&self.bounding_box.to_string(), 4), indent(&self.object_to_world().to_string(), 4))
    }
}

impl Sphere{
    pub fn new(prop_list: PropertyList) -> Self {
        let radius = prop_list.get_float("radius", 1f32);
        let z_min = prop_list.get_float("z_min", -1f32).clamp(-radius, radius);
        let z_max = prop_list.get_float("z_max", 1f32).clamp(-radius, radius);
        let phi_max = prop_list.get_float("phi_max", 360f32).clamp(0f32, 360f32).to_radians();

        let bounding_box = Bounds3f::init(
        &Point3f::init([-radius, -radius, z_min]),
        &Point3f::init([radius, radius, z_max])
        );

        Sphere{
            radius,
            z_min, z_max,
            theta_min: (z_min / radius).acos().clamp(-1f32, 1f32),
            theta_max: (z_max / radius).acos().clamp(-1f32, 1f32),
            phi_max,
            object_to_world: Transform::new(), world_to_object: Transform::new(),
            bounding_box,
            reverse_orientation: prop_list.get_bool("reverse_orientation", false)
        }
    }
}

register_struct!("sphere", create_sphere);