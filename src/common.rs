pub use lazy_static::lazy_static;
pub use std::any::Any;
pub use std::rc::Rc;
pub use std::{cell::RefCell, sync::{Arc, Mutex}};

pub use crate::factory::{register_lead_object, create_lead_object};

pub use crate::accel::*;
pub use crate::camera::*;
pub use crate::filter::*;
pub use crate::geometry::*;
pub use crate::integrator::*;
pub use crate::light::*;
pub use crate::material::*;
pub use crate::medium::*;
pub use crate::sampler::*;
pub use crate::scene::*;
pub use crate::shapes::*;
pub use crate::spectrum::*;

pub use crate::utils::*;
pub use crate::macros::*;


pub fn indent(input: &str, spaces: usize) -> String {
    let indentation = " ".repeat(spaces);
    input
        .lines()
        .map(|line| format!("{}{}", indentation, line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub const EPSILON: f32 = 10e-4;
pub const INFINITY: f32 = f32::INFINITY;
pub const ONE_MINUS_EPSILON: f32 = 1.0 - EPSILON;

pub const M_PI: f32 = 3.14159265359;
pub const M_INV_PI: f32 = 0.31830988618;

pub type Spectrum = RGBSpectrum;