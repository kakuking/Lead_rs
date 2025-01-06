use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt::Display;
use std::str::FromStr;

use super::vector2::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Point2<T> 
where T: Float + Copy{
    pub x: T,
    pub y: T
}

impl<T> Point2<T>
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

    pub fn init_copy(other: &Self) -> Self {
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

    pub fn distance(p1: &Self, p2: &Self) -> T {
        (*p1 - *p2).length()
    }

    pub fn distance_sqr(p1: &Self, p2: &Self) -> T {
        (*p1 - *p2).length_sqr()
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn equal(p1: &Self, p2: &Self) -> bool {
        p1.x == p2.x && p1.y == p2.y
    }
}

impl_operator_2!(Point2<T>, Add, add, +, Point2<T>);
impl_operator_2!(Point2<T>, Sub, sub, -, Vector2<T>);
impl_operator_2!(Point2<T>, Mul, mul, *, Point2<T>);
impl_operator_2!(Point2<T>, Div, div, /, Point2<T>);
impl_operator_unary_2!(Point2<T>, Neg, neg, -);
impl_operator_inplace_2!(Point2<T>, AddAssign, add_assign, +=);
impl_operator_inplace_2!(Point2<T>, SubAssign, sub_assign, -=);
impl_operator_inplace_2!(Point2<T>, MulAssign, mul_assign, *=);
impl_operator_inplace_2!(Point2<T>, DivAssign, div_assign, /=);
impl<T> Index<usize> for Point2<T>
where
    T: Copy + Float,
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds while accessing vec2!")
        }
    }
}

impl<T> IndexMut<usize> for Point2<T>
where
    T: Copy + Float,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds while accessing vec2!")
        }
    }
}

impl<T> Mul<T> for Point2<T>
    where
    T: Mul<Output = T> + Float + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Point2<T>
    where
    T: Div<Output = T> + Float + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        assert!(rhs != T::zero(), "RHS cannot be zero for division!");
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl<T> Add<Vector2<T>> for Point2<T> 
    where
    T: Add<Output = T> + Float + Copy {
        type Output = Self;

        fn add(self, rhs: Vector2<T>) -> Self::Output {
            Self {
                x: self.x + rhs[0],
                y: self.y + rhs[1],
            }
        }
}

impl<T> Sub<Vector2<T>> for Point2<T> 
    where
    T: Sub<Output = T> + Float + Copy {
        type Output = Self;

        fn sub(self, rhs: Vector2<T>) -> Self::Output {
            Self {
                x: self.x - rhs[0],
                y: self.y - rhs[1],
            }
        }
}
pub type Point2i = Point2<i32>;
pub type Point2f = Point2<f32>;
pub type Point2d = Point2<f64>;