pub use lazy_static::lazy_static;
pub use std::any::Any;

pub use crate::factory::{register_lead_object, create_lead_object};

pub use crate::traits::lead_object::*;
pub use crate::traits::property_list::*;
pub use crate::traits::shape::*;

pub use crate::macros::*;

pub use crate::utils::vector2::{Vector2f, Vector2d};
pub use crate::utils::vector3::{Vector3f, Vector3d, coordinate_system};

pub use crate::impls::scene::*;

pub fn indent(input: &str, spaces: usize) -> String {
    let indentation = " ".repeat(spaces);
    input
        .lines()
        .map(|line| format!("{}{}", indentation, line))
        .collect::<Vec<_>>()
        .join("\n")
}
