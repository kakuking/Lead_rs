use crate::common::*;

#[derive(Debug, Clone)]
pub struct BoxFilter {
    radius: Vector2f,
    inv_radius: Vector2f
}

impl Filter for BoxFilter {
    fn radius(&self) -> Vector2f { self.radius }
    fn inv_radius(&self) -> Vector2f { self.inv_radius }
    fn set_radius(&mut self, n: &Vector2f) { self.radius = n.clone(); }
    fn set_inv_radius(&mut self, n: &Vector2f) { self.inv_radius = n.clone(); }
    // Only called within the radius of the filter so we are good
    fn evaluate(&self, _p: &Point2f) -> f32 { 1.0 }
}

impl BoxFilter {
    pub fn new(radius: &Vector2f) -> Self {
        Self {
            radius: radius.clone(),
            inv_radius: Vector2f::init([1.0/radius.x(), 1.0/radius.y()])
        }
    }
}