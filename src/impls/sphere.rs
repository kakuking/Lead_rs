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

    fn intersect(&self, ray: &Ray, t_hit: &mut f32, _its:  &mut SurfaceInteraction) -> bool {
        let phi: f32;
        let p_hit: Point3f;

        let o_obj: Point3f = &self.world_to_object() * ray.o;
        let d_obj: Vector3f = &self.world_to_object() * ray.d;

        let d_sqr = Vector3f::dot(&d_obj, &d_obj);
        let o_sqr = o_obj.x()*o_obj.x() + o_obj.y()*o_obj.y() + o_obj.z()*o_obj.z();
        let c_sqr = 0f32;   // of center

        let oc = 0f32;  // o dot center
        let r2 = self.radius * self.radius;
        let o_minus_c: Vector3f = o_obj - Point3f::new();
        let d_dot_o_minus_c = Vector3f::dot(&o_minus_c, &d_obj);

        let a = d_sqr;
        let b = 2f32*d_dot_o_minus_c;
        let c = c_sqr + o_sqr - 2f32*oc - r2;
        let det = b*b - 4f32*a*c;

        if det < 0f32 {
            return false;
        }

        let root_det = det.sqrt();

        let t_less = 0.5 * (-b - root_det) / a;
        let t_more = 0.5 * (-b + root_det) / a;

        // its outside acceptable range
        if t_more < ray.t_min || t_less > ray.t_max {
            return false;
        }

        let mut flag = false;

        if t_more >= ray.t_min && t_more <= ray.t_max {
            *t_hit = t_more;
            flag = true;
        }

        if t_less >= ray.t_min && t_less <= ray.t_max {
            *t_hit = t_less;
            flag = true;
        }

        if !flag {
            return false;
        }

        let p = o_obj + d_obj * (*t_hit);

        let theta = (p.z() / self.radius).acos();
        let phi = p.y().signum() * (p.x() / (p.x()*p.x() + p.y()*p.y()).sqrt()).acos();

        // Check if within bounds
        if phi > self.phi_max || theta > self.theta_max || theta < self.theta_min {
            return false;
        }

        let u: f32 = phi / self.phi_max;
        let v: f32 = (theta - self.theta_min) / (self.theta_max - self.theta_min);

        // let z_radius = 



        true
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

    pub fn calculate_uv(u: &mut f32, v: &mut f32, p: Point3f) {
        let theta = p.x().atan2(p.y()) + M_PI;
        let length = Point3f::dot(&p, &p);
        let phi = (p.z()/length).acos();

        *u = theta / (2.0*M_PI);
        *v = phi * M_INV_PI;
    }
}

register_struct!("sphere", create_sphere);