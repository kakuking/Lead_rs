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
    LeadObject::Shape(Arc::new(sphere))
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

    fn intersect(&self, ray: &Ray, t_hit: &mut f32, its:  &mut SurfaceInteraction) -> bool {
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
        
        let mut r_1: Option<f32> = None;
        let mut r_2: Option<f32> = None;
        if !Solver::quadratic(a, b, c, &mut r_1, &mut r_2) {
            return false;
        }

        let t_less = r_1.unwrap_or(-INFINITY);
        let t_more = r_2.unwrap_or(-INFINITY);

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
        let uv = Point2f::init([u, v]);
        
        let dpdu = Vector3f::init([-self.phi_max * p.y(), self.phi_max * p.x(), 0f32]);
        let dpdv = Vector3f::init([p.z() * phi.cos(), p.z() * phi.sin(), -self.radius * theta.sin()]) * (self.theta_max - self.theta_min);

        let d2_pduu = Vector3f::init([p.x(), p.y(), 0f32]) * -self.phi_max * self.phi_max;
        let d2_pduv = Vector3f::init([-phi.sin(), phi.cos(), 0.0]) * -(self.theta_max - self.theta_min) * p.z() * self.phi_max;
        let d2_pdvv = Vector3f::init([p.x(), p.y(), p.z()]) * -(self.theta_max - self.theta_min)*(self.theta_max - self.theta_min);
        
        let big_e = Vector3f::dot(&dpdu, &dpdu);
        let big_f = Vector3f::dot(&dpdu, &dpdv);
        let big_g = Vector3f::dot(&dpdv, &dpdv);
        let big_n = Vector3f::normalize(&Vector3f::cross(&dpdu, &dpdv));
        let e = Vector3f::dot(&big_n, &d2_pduu);
        let f = Vector3f::dot(&big_n, &d2_pduv);
        let g = Vector3f::dot(&big_n, &d2_pdvv);

        let inv_egf2 = 1.0 / (big_e*big_g - big_f*big_f);
        let dndu = Normal3f::init_vector(&(dpdu * (f*big_f - e*big_g)*inv_egf2 + dpdv*(e*big_f - f*big_e)*inv_egf2));
        let dndv = Normal3f::init_vector(&(dpdu * (g*big_f - f*big_g)*inv_egf2 + dpdv*(f*big_f - f*big_e)*inv_egf2));

        let obj_its = SurfaceInteraction::init(p, uv, -ray.d, dpdu, dpdv, dndu, dndv, *t_hit);

        *its = &self.object_to_world * &obj_its;

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

        let object_to_world = prop_list.get_transform();

        Sphere{
            radius,
            z_min, z_max,
            theta_min: (z_min / radius).acos().clamp(-1f32, 1f32),
            theta_max: (z_max / radius).acos().clamp(-1f32, 1f32),
            phi_max,
            world_to_object: object_to_world.inverse(),
            object_to_world: object_to_world,
            bounding_box,
            reverse_orientation: prop_list.get_bool("reverse_orientation", false),
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