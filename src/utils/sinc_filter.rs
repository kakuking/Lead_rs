use crate::common::*;

#[derive(Debug, Clone)]
pub struct SincFilter {
    radius: Vector2f,
    inv_radius: Vector2f,

    tau: f32
}

impl Filter for SincFilter {
    fn radius(&self) -> Vector2f { self.radius }
    fn inv_radius(&self) -> Vector2f { self.inv_radius }
    fn set_radius(&mut self, n: &Vector2f) { self.radius = n.clone(); }
    fn set_inv_radius(&mut self, n: &Vector2f) { self.inv_radius = n.clone(); }
    fn evaluate(&self, p: &Point2f) -> f32 { 
        self.windowed_sinc(p.x(), self.radius.x()) * self.windowed_sinc(p.y(), self.radius.y())
    }
}

impl SincFilter {
    pub fn new(radius: &Vector2f, tau: f32) -> Self {
        Self {
            radius: radius.clone(),
            inv_radius: Vector2f::init([1.0/radius.x(), 1.0/radius.y()]),
            tau
        }
    }

    fn sinc(&self, x: f32) -> f32 {
        let x = x.abs();
        if x < EPSILON {
            return 0.0;
        }

        (M_PI * x).sin() / x * M_INV_PI
    }

    fn windowed_sinc(&self, x: f32, radius: f32) -> f32 {
        let x = x.abs();

        if x > radius {
            return 0.0;
        }

        let lanczos = self.sinc(x / self.tau);
        self.sinc(x) * lanczos
    }
}