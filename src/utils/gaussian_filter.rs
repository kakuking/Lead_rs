use crate::common::*;

#[derive(Debug, Clone)]
pub struct GaussianFilter {
    radius: Vector2f,
    inv_radius: Vector2f,

    alpha: f32,
    exp_x: f32, exp_y: f32
}

impl Filter for GaussianFilter {
    fn radius(&self) -> Vector2f { self.radius }
    fn inv_radius(&self) -> Vector2f { self.inv_radius }
    fn set_radius(&mut self, n: &Vector2f) { self.radius = n.clone(); }
    fn set_inv_radius(&mut self, n: &Vector2f) { self.inv_radius = n.clone(); }
    // Only called within the radius of the filter so we are good
    fn evaluate(&self, p: &Point2f) -> f32 {
        self.gaussian_filter(p.x(), self.exp_x) * self.gaussian_filter(p.y(), self.exp_y)
    }
}

impl GaussianFilter {
    pub fn new(alpha: f32, radius: &Vector2f) -> Self {
        Self {
            radius: radius.clone(),
            inv_radius: Vector2f::init([1.0/radius.x(), 1.0/radius.y()]),
            alpha: alpha,
            exp_x: (-alpha * radius.x() * radius.x()).exp(),
            exp_y: (-alpha * radius.y() * radius.y()).exp(),
        }
    }

    fn gaussian_filter(&self, d: f32, exp_v: f32) -> f32 {
        0f32.max((-self.alpha * d * d).exp() - exp_v)
    }
}