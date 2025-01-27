use crate::common::*;
use std::fmt::Debug;

pub trait Filter: Debug {
    fn radius(&self) -> Vector2f;
    fn inv_radius(&self) -> Vector2f;
    
    fn set_radius(&mut self, n: &Vector2f);
    fn set_inv_radius(&mut self, n: &Vector2f);

    fn init(&mut self, radius: &Vector2f) { 
        self.set_radius(radius);
        self.set_inv_radius(&Vector2f::init([1.0 / radius.x(), 1.0 / radius.y()]));
    }
    fn evaluate(&self, p: &Point2f) -> f32;
}