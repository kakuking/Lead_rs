use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Display;
use std::str::FromStr;

use super::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> 
where T: Float + Copy{
    x: T,
    y: T
}

impl<T> Vector2<T>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{
    pub fn new() -> Self {
        Self {
            x: T::zero(),
            y: T::zero()
        }
    }
    
    pub fn init(x: T, y: T) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
    
    pub fn init_one(x: T) -> Self {
        Self {
            x: x,
            y: x,
        }
    }

    pub fn init_copy(other: &Vector2<T>) -> Self {
        if other.has_nan() {
            println!("Vector tryingto be copied has Nans!");
            return Self::new();
        }

        Self {
            x: other.x,
            y: other.y
        }
    }

    pub fn init_string(input: String) -> Self {
        let cleaned = input.trim_start().trim_end();
        let parts: Vec<&str> = cleaned.split(',').collect();

        match parts.len() {
            1 => {
                let x = parts[0].trim().parse::<T>().expect("Failed to parse x");
                Self::init_one(x)
            }
            2 => {
                let x = parts[0].trim().parse::<T>().expect("Failed to parse x");
                let y = parts[0].trim().parse::<T>().expect("Failed to parse y");
                Self::init(x, y)
            }
            _ => {
                panic!("Failed to parse input string {input} to vector2");
            }
        }
    }

    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn length_sqr(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn to_string(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }

    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y*v2.y
    }

    pub fn abs_dot(v1: &Self, v2: &Self) -> T {
        Self::dot(v1, v2).abs()
    }

    pub fn cross(v1: &Self, v2: &Self) -> Vector3<T> {
        let t1: Vector3<T> = Vector3::init(v1.x, v1.y, T::zero());
        let t2: Vector3<T> = Vector3::init(v2.x, v2.y, T::zero());

        Vector3::cross(&t1, &t2)
    }

    pub fn normalize(v: &Self) -> Self {
        let l = v.length();
        Self::init(v.x / l, v.y / l)
    }

    pub fn min_component(&self) -> T {
        self.x.min(self.y)
    }

    pub fn max_component(&self) -> T {
        self.x.max(self.y)
    }

    pub fn min_dimension(&self) -> usize {
        if self.x < self.y {
            return 0usize;
        }
        return 1usize;
    }

    pub fn max_dimension(&self) -> usize {
        if self.x > self.y {
            return 0usize;
        }
        return 1usize;
    }

    pub fn min(v1: &Self, v2: &Self) -> Self {
        Self::init(
            v1.x.min(v2.x), 
            v1.y.min(v2.y)
        )
    }

    pub fn max(v1: &Self, v2: &Self) -> Self {
        Self::init(
            v1.x.max(v2.x), 
            v1.y.max(v2.y)
        )
    }

    pub fn permute(v: &Self, x: usize, y: usize) -> Self {
        Self::init(v[x], v[y])
    }
}

impl_operator_2!(Vector2<T>, Add, add, +);
impl_operator_2!(Vector2<T>, Sub, sub, -);
impl_operator_2!(Vector2<T>, Mul, mul, *);
impl_operator_2!(Vector2<T>, Div, div, /);
impl_operator_unary_2!(Vector2<T>, Neg, neg, -);
impl_operator_inplace_2!(Vector2<T>, AddAssign, add_assign, +=);
impl_operator_inplace_2!(Vector2<T>, SubAssign, sub_assign, -=);
impl_operator_inplace_2!(Vector2<T>, MulAssign, mul_assign, *=);
impl_operator_inplace_2!(Vector2<T>, DivAssign, div_assign, /=);
impl<T> Index<usize> for Vector2<T>
where
    T: Copy + Float,
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out f bounds while accessing vec2!")
        }
    }
}

impl<T> IndexMut<usize> for Vector2<T>
where
    T: Copy + Float,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out f bounds while accessing vec2!")
        }
    }
}

pub type Vector2f = Vector2<f32>;
pub type Vector2d = Vector2<f64>;