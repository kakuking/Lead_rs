use crate::common::*;

#[derive(Debug, Clone)]
pub struct TriangleFilter {
    radius: Vector2f,
    inv_radius: Vector2f
}

impl Filter for TriangleFilter {
    fn radius(&self) -> Vector2f { self.radius }
    fn inv_radius(&self) -> Vector2f { self.inv_radius }
    fn set_radius(&mut self, n: &Vector2f) { self.radius = n.clone(); }
    fn set_inv_radius(&mut self, n: &Vector2f) { self.inv_radius = n.clone(); }
    // Only called within the radius of the filter so we are good
    fn evaluate(&self, p: &Point2f) -> f32 {
        let x_part = 0f32.max(self.radius.x() - p.x().abs());
        let y_part = 0f32.max(self.radius.x() - p.y().abs());
        x_part * y_part
    }
}

impl TriangleFilter{
    pub fn new(radius: &Vector2f) -> Self {
        Self {
            radius: radius.clone(),
            inv_radius: Vector2f::init([1.0/radius.x(), 1.0/radius.y()])
        }
    }
}