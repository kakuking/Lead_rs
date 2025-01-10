use crate::common::*;

#[derive(Clone)]
pub struct Shading {
    pub n: Normal3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal3f,
    pub dndv: Normal3f
}

impl Shading {
    pub fn new() -> Self {
        Self {
            n: Normal3f::new(),
            dpdu: Vector3f::new(), dpdv: Vector3f::new(), dndu: Normal3f::new(), dndv: Normal3f::new()
        }
    }
}

pub trait Interaction {
    fn p(&self) -> Point3f;
    fn t(&self) -> f32;
    fn wo(&self) -> Vector3f;
    fn n(&self) -> Normal3f;
    fn medium_interface(&self) -> &MediumInterface;

    fn is_surface_interaction(&self) -> bool {
        Normal3f::equal(&self.n(), &Normal3f::new())
    }

    fn spawn_ray(&self, d: &Vector3f) -> Ray {
        Ray::init(&self.p(), d, EPSILON, INFINITY)
    }

    fn spawn_ray_to(&self, p: &Point3f) -> Ray {
        let d = Vector3f::normalize(&(*p - self.p()));
        self.spawn_ray(&d)
    }

    fn spawn_ray_to_interaction(&self, it: &dyn Interaction) -> Ray {
        let o = self.p();
        let d = it.p() - o;

        Ray::init(&o, &d, EPSILON, INFINITY)
    }

    fn is_medium_interaction(&self) -> bool {
        !self.is_surface_interaction()
    }

    fn get_medium(&self) -> Rc<Medium> {
        assert!(&self.medium_interface().is_homogeneous(), "Inside and outside media are not the same, provide a reference vector!");

        self.medium_interface().inside.clone()
    }

    fn get_medium_vector(&self, v: &Vector3f) -> Rc<Medium> {
        return if Normal3f::dot(&self.n(), &v) > 0.0 {self.medium_interface().inside.clone()} else {self.medium_interface().outside.clone()}
    }
}

