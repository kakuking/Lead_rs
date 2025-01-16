use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, Neg};
use std::fmt::Display;
use std::str::FromStr;
use derive_more::{Index, IndexMut};

use super::vector::Vector;

#[derive(Debug, Clone, Copy, Index, IndexMut)]
pub struct Normal<T: Float + Copy, const N: usize> {
    #[index]
    #[index_mut]
    direction: [T; N]
}

impl<T, const N: usize> Normal<T, N>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{    
    pub fn new() -> Self {
        Self {
            direction: [T::zero(); N]
        }
    }
    
    pub fn init(direction: [T; N]) -> Self {
        Self {
            direction: direction
        }
    }
    
    pub fn init_one(x: T) -> Self {
        Self {
            direction: [x; N]
        }
    }

    pub fn init_copy(other: &Self) -> Self {
        if other.has_nan() {
            println!("Normal tryingto be copied has Nans!");
            return Self::new();
        }

        Self {
            direction: other.direction.clone()
        }
    }

    pub fn init_vector(other: &Vector<T, N>) -> Self {
        let mut direction = [T::zero(); N];
        for i in 0..N {
            direction[i] = other[i];
        }
        Self {
            direction
        }
    }

    pub fn x(&self) -> T {
        self.direction[0]
    }

    pub fn y(&self) -> T {
        self.direction[1]
    }

    pub fn z(&self) -> T {
        self.direction[2]
    }

    pub fn init_string(input: String) -> Self {
        let cleaned = input.trim_start().trim_end();
        let parts: Vec<&str> = cleaned.split(',').collect();

        match parts.len() {
            1 => {
                let x = parts[0].trim().parse::<T>().expect("Failed to parse x");
                Self::init_one(x)
            }
            _ => {
                if parts.len() != N {
                    panic!(
                        "Cannot initialize a Vector<{N}> with {} values from the input string: {}",
                        parts.len(),
                        input
                    );
                }
        
                let mut values = [T::zero(); N]; // Create an uninitialized array of size N
                for (i, part) in parts.iter().enumerate() {
                    values[i] = part
                        .trim()
                        .parse::<T>()
                        .expect(&format!("Failed to parse value at index {i}"));
                }
        
                Self::init(values)
            }
        }
    }

    pub fn has_nan(&self) -> bool {
        self.x().is_nan() || self.y().is_nan() || self.z().is_nan()
    }

    pub fn length_sqr(&self) -> T {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> T {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x(), self.y(), self.z())
    }

    pub fn dot(v1: &Self, v2: &Vector<T, 3usize>) -> T {
        v1.x()*v2.x() + v1.y()*v2.y() + v1.z()*v2.z()
    }

    pub fn abs_dot(v1: &Self, v2: &Vector<T, 3usize>) -> T {
        Self::dot(v1, v2).abs()
    }

    pub fn normalize(v: &Self) -> Self {
        let l = v.length();
        Self::init(v.direction.map(|x| x/l))
    }

    pub fn min_component(&self) -> T {
        self.x().min(self.y().min(self.z()))
    }

    pub fn max_component(&self) -> T {
        self.x().max(self.y().max(self.z()))
    }

    pub fn min_dimension(&self) -> usize {
        if self.x() < self.y() {
            if self.x() < self.z() {
                return 0usize;
            } 
            return 2usize;
        }
        
        if self.y() < self.x() {
            if self.y() < self.z() {
                return 1usize;
            }
            return  2usize;
        }

        return 0usize;
    }

    pub fn max_dimension(&self) -> usize {
        if self.x() > self.y() {
            if self.x() > self.z() {
                return 0usize;
            } 
            return 2usize;
        }
        
        if self.y() > self.x() {
            if self.y() > self.z() {
                return 1usize;
            }
            return  2usize;
        }

        return 0usize;
    }

    pub fn min(v1: &Self, v2: &Self) -> Self {
        let mut new_min = v1.direction.clone();
        for i in 0..N {
            new_min[i] = new_min[i].min(v2[i]);
        }
        Self::init(
            new_min
        )
    }

    pub fn max(v1: &Self, v2: &Self) -> Self {
        let mut new_min = v1.direction.clone();
        for i in 0..N {
            new_min[i] = new_min[i].max(v2[i]);
        }
        Self::init(
            new_min
        )
    }

    pub fn permute(v: &Self, x: [usize; N]) -> Self {
        if x.iter().any(|&i| i >= N) {
            panic!("Index out of bounds for permutation");
        }

        Self::init(x.map(|i| v.direction[i]))
    }

    pub fn faceforward(n: &Self, v: &Vector<T, 3usize>) -> Self {
        if Self::dot(n, v) < T::zero() {
            return -n.clone();
        }
        return n.clone();
    }

    pub fn equal(p1: &Self, p2: &Self) -> bool {
        p1.x() == p2.x() && p1.y() == p2.y() && p1.z() == p2.z()
    }
}

impl_operator!(Normal<T, const N: usize>, Add, add, +, Normal<T, const N: usize>);
impl_operator!(Normal<T, const N: usize>, Sub, sub, -, Normal<T, const N: usize>);
impl_operator!(Normal<T, const N: usize>, Mul, mul, *, Normal<T, const N: usize>);
impl_operator_unary!(Normal<T, const N: usize>, Neg, neg, -);
impl_operator_inplace!(Normal<T, const N: usize>, AddAssign, add_assign, +=);
impl_operator_inplace!(Normal<T, const N: usize>, SubAssign, sub_assign, -=);
impl_operator_inplace!(Normal<T, const N: usize>, MulAssign, mul_assign, *=);

impl<T> Mul<T> for Normal<T, 3usize>
where 
    T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::init([self.x() * rhs,self.y() * rhs,self.z() * rhs])
    }
}

impl<T> Div<T> for Normal<T, 3usize>
where 
    T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        assert!(rhs != T::zero(), "RHS cannot be zero for division!");
        Self::init([self.x() / rhs,self.y() / rhs,self.z() / rhs])

    }
}

pub type Normal3i = Normal<i32, 3usize>;
pub type Normal3f = Normal<f32, 3usize>;
pub type Normal3d = Normal<f64, 3usize>;