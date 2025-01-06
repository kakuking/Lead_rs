use crate::common::*;


#[derive(Debug, Clone)]
pub struct Interaction {
    pub p: Point3f,
    pub t: f32,
    pub wo: Vector3f,
    pub n: Normal3f,
    pub medium_interface: MediumInterface
}

impl Interaction {
    pub fn new() -> Self {
        Self {
            p: Point3f::new(),
            t: 0f32,
            wo: Vector3f::new(),
            n: Normal3f::new(),
            medium_interface: MediumInterface::new()
        }
    }

    pub fn init(p: &Point3f, n: &Normal3f, wo: &Vector3f, t: f32, medium_interface: &MediumInterface) -> Self {
        Self {
            p: p.clone(),
            t: t,
            wo: wo.clone(),
            n: n.clone(),
            medium_interface: medium_interface.clone()
        }
    }

    pub fn is_surface_interaction(&self) -> bool {
        Normal3f::equal(&self.n, &Normal3f::new())
    }

    pub fn spawn_ray(&self, d: &Vector3f) -> Ray {
        Ray::init(&self.p, d, EPSILON, INFINITY)
    }

    pub fn spawn_ray_to(&self, p: &Point3f) -> Ray {
        let d = Vector3f::normalize(&(*p - self.p));
        self.spawn_ray(&d)
    }

    pub fn spawn_ray_to_interaction(&self, it: &Self) -> Ray {
        let o = self.p;
        let d = it.p - o;

        Ray::init(&o, &d, EPSILON, INFINITY)
    }

    pub fn is_medium_interaction(&self) -> bool {
        !self.is_surface_interaction()
    }

    pub fn get_medium(&self) -> Rc<Medium> {
        assert!(&self.medium_interface.is_homogeneous(), "Inside and outside media are not the same, provide a reference vector!");

        self.medium_interface.inside.clone()
    }

    pub fn get_medium_vector(&self, v: &Vector3f) -> Rc<Medium> {
        return if Normal3f::dot(&self.n, &v) > 0.0 {self.medium_interface.inside.clone()} else {self.medium_interface.outside.clone()}
    }
}