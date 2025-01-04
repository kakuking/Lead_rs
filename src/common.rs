pub use lazy_static::lazy_static;
pub use std::any::Any;

pub use crate::factory::{register_lead_object, create_lead_object};

pub use crate::traits::lead_object::*;
pub use crate::traits::property_list::*;
pub use crate::traits::shape::*;

pub use crate::impls::scene::*;

pub fn indent(input: &str, spaces: usize) -> String {
    let indentation = " ".repeat(spaces);
    input
        .lines()
        .map(|line| format!("{}{}", indentation, line))
        .collect::<Vec<_>>()
        .join("\n")
}

#[macro_export]
macro_rules! register_struct {
    ($inp1:expr, $inp2:expr) => {
        #[ctor::ctor]
        fn register_sphere() {
            register_lead_object($inp1, $inp2);
        }
    };
}

pub use register_struct;