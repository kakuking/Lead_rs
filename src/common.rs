pub use lazy_static::lazy_static;
pub use std::any::Any;
pub use std::rc::Rc;

pub use crate::factory::{register_lead_object, create_lead_object};

pub use crate::traits::lead_object::*;
pub use crate::traits::property_list::*;
pub use crate::traits::shape::*;
pub use crate::traits::medium::*;
pub use crate::traits::interaction::{Interaction, Shading};

pub use crate::macros::*;

pub use crate::utils::point::{Point, Point2i, Point2f, Point2d, Point3i, Point3f, Point3d};
// pub use crate::utils::vector2::{Vector2i, Vector2f, Vector2d};
pub use crate::utils::vector::{Vector, Vector2i, Vector2f, Vector2d, Vector3i, Vector3f, Vector3d, coordinate_system};
pub use crate::utils::normal::{Normal3i, Normal3f, Normal3d};
pub use crate::utils::ray::{Ray, RayDifferential};
pub use crate::utils::bounding_box::{Bounds2i, Bounds2f, Bounds3i, Bounds3f};
pub use crate::utils::matrix::Matrix4x4;
pub use crate::utils::transform::Transform;

pub use crate::impls::scene::*;
pub use crate::impls::interaction::*;

pub fn indent(input: &str, spaces: usize) -> String {
    let indentation = " ".repeat(spaces);
    input
        .lines()
        .map(|line| format!("{}{}", indentation, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub const EPSILON: f32 = 0.0001;
pub const INFINITY: f32 = f32::INFINITY;
pub const M_PI: f32 = 3.14159265359;
pub const M_INV_PI: f32 = 0.31830988618;