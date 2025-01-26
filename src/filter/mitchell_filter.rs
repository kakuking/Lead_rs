use crate::common::*;

#[derive(Debug, Clone)]
pub struct MitchellFilter {
    radius: Vector2f,
    inv_radius: Vector2f,

    b: f32, c: f32
}

impl Filter for MitchellFilter {
    fn radius(&self) -> Vector2f { self.radius }
    fn inv_radius(&self) -> Vector2f { self.inv_radius }
    fn set_radius(&mut self, n: &Vector2f) { self.radius = n.clone(); }
    fn set_inv_radius(&mut self, n: &Vector2f) { self.inv_radius = n.clone(); }
    fn evaluate(&self, p: &Point2f) -> f32 {
        self.mitchell_1d(p.x() * self.inv_radius.x()) * self.mitchell_1d(p.y() * self.inv_radius.y())
    }
}

impl MitchellFilter {
    pub fn new(radius: &Vector2f, b: f32, c: f32) -> Self {
        Self {
            radius: radius.clone(),
            inv_radius: Vector2f::init([1.0/radius.x(), 1.0/radius.y()]),
            b, c
        }
    }

    fn mitchell_1d(&self, x: f32) -> f32 {
        let x = 2.0*x.abs();
        if x > 1.0 {
            ((-self.b - 6.0*self.c) * x*x*x + (6.0*self.b + 30.0*self.c) * x*x +
            (-12.0*self.b - 48.0*self.c) * x + (8.0*self.b + 24.0*self.c)) * (1.0/6.0)
        } else {
            ((12.0 - 9.0*self.b - 6.0*self.c) * x*x*x + 
            (-18.0 + 12.0*self.b + 6.0*self.c) * x*x +
            (6.0 - 2.0*self.b)) * (1.0/6.0)
        }
    }
}