use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index, IndexMut};
use std::fmt::Display;
use std::str::FromStr;

use super::vector3::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Point3<T: Float + Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Point3<T>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{    
    pub fn new() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
    
    pub fn init(x: T, y: T, z: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
    
    pub fn init_one(x: T) -> Self {
        Self {
            x: x,
            y: x,
            z: x
        }
    }

    pub fn init_copy(other: &Point3<T>) -> Self {
        if other.has_nan() {
            println!("Vector tryingto be copied has Nans!");
            return Self::new();
        }

        Self {
            x: other.x,
            y: other.y,
            z: other.z
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
            3 => {
                let x = parts[0].trim().parse::<T>().expect("Failed to parse x");
                let y = parts[0].trim().parse::<T>().expect("Failed to parse y");
                let z = parts[0].trim().parse::<T>().expect("Failed to parse z");
                Self::init(x, y, z)
            }
            _ => {
                panic!("Failed to parse input string {input} to point3");
            }
        }
    }

    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn length_sqr(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(v: &Self) -> Self {
        let l = v.length();
        Self::init(v.x / l, v.y / l, v.z / l)
    }

    pub fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }

    pub fn min_component(&self) -> T {
        self.x.min(self.y.min(self.z))
    }

    pub fn max_component(&self) -> T {
        self.x.max(self.y.max(self.z))
    }

    pub fn min_dimension(&self) -> usize {
        if self.x < self.y {
            if self.x < self.z {
                return 0usize;
            } 
            return 2usize;
        }
        
        if self.y < self.x {
            if self.y < self.z {
                return 1usize;
            }
            return  2usize;
        }

        return 0usize;
    }

    pub fn max_dimension(&self) -> usize {
        if self.x > self.y {
            if self.x > self.z {
                return 0usize;
            } 
            return 2usize;
        }
        
        if self.y > self.x {
            if self.y > self.z {
                return 1usize;
            }
            return  2usize;
        }

        return 0usize;
    }

    pub fn min(v1: &Self, v2: &Self) -> Self {
        Self::init(
            v1.x.min(v2.x), 
            v1.y.min(v2.y),
            v1.z.min(v2.z)
        )
    }

    pub fn max(v1: &Self, v2: &Self) -> Self {
        Self::init(
            v1.x.max(v2.x), 
            v1.y.max(v2.y),
            v1.z.max(v2.z)
        )
    }

    pub fn permute(v: &Self, x: usize, y: usize, z: usize) -> Self {
        Self::init(v[x], v[y], v[z])
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
            z: self.z.floor(),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    
    pub fn equal(p1: &Self, p2: &Self) -> bool {
        p1.x == p2.x && p1.y == p2.y && p1.z == p2.z
    }
}

impl_operator_3!(Point3<T>, Add, add, +, Point3<T>);
impl_operator_3!(Point3<T>, Sub, sub, -, Vector3<T>);
impl_operator_3!(Point3<T>, Mul, mul, *, Point3<T>);
impl_operator_3!(Point3<T>, Div, div, /, Point3<T>);
impl_operator_unary_3!(Point3<T>, Neg, neg, -);
impl_operator_inplace_3!(Point3<T>, AddAssign, add_assign, +=);
impl_operator_inplace_3!(Point3<T>, SubAssign, sub_assign, -=);
impl_operator_inplace_3!(Point3<T>, MulAssign, mul_assign, *=);
impl_operator_inplace_3!(Point3<T>, DivAssign, div_assign, /=);
impl<T> Index<usize> for Point3<T>
where
    T: Copy + Float,
{
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds while accessing vec3!")
        }
    }
}

impl<T> IndexMut<usize> for Point3<T>
where
    T: Copy + Float,
{
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds while accessing vec3!")
        }
    }
}

impl<T> Mul<T> for Point3<T>
    where
    T: Mul<Output = T> + Float + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Point3<T>
    where
    T: Div<Output = T> + Float + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        assert!(rhs != T::zero(), "RHS cannot be zero for division!");
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Add<Vector3<T>> for Point3<T> 
    where
    T: Add<Output = T> + Float + Copy {
        type Output = Self;

        fn add(self, rhs: Vector3<T>) -> Self::Output {
            Self {
                x: self.x + rhs[0],
                y: self.y + rhs[1],
                z: self.z + rhs[2],
            }
        }
}

impl<T> Sub<Vector3<T>> for Point3<T> 
    where
    T: Sub<Output = T> + Float + Copy {
        type Output = Self;

        fn sub(self, rhs: Vector3<T>) -> Self::Output {
            Self {
                x: self.x - rhs[0],
                y: self.y - rhs[1],
                z: self.z - rhs[2],
            }
        }
}

pub type Point3i = Point3<i32>;
pub type Point3f = Point3<f32>;
pub type Point3d = Point3<f64>;
