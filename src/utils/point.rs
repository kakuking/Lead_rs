use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::ops::{Mul, MulAssign, Div, Neg};
use std::fmt::Display;
use std::str::FromStr;
use derive_more::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Index, IndexMut)]
pub struct Point<T: Float + Copy, const N: usize> {
    #[index]
    #[index_mut]
    coordinates: [T; N]
}

impl<T, const N: usize> Point<T, N>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug, 
{    
    pub fn new() -> Self {
        Self {
            coordinates: [T::zero(); N]
        }
    }
    
    pub fn init(coords: [T; N]) -> Self {
        Self {
            coordinates: coords
        }
    }
    
    pub fn init_one(x: T) -> Self {
        Self {
            coordinates: [x; N]
        }
    }

    pub fn init_copy(other: &Point<T, N>) -> Self {
        Self {
            coordinates: other.coordinates
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
            _ => {
                if parts.len() != N {
                    panic!(
                        "Cannot initialize a Point<{N}> with {} values from the input string: {}",
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

    pub fn x(&self) -> T {
        self.coordinates[0]
    }

    pub fn y(&self) -> T {
        self.coordinates[1]
    }

    pub fn z(&self) -> T {
        assert!(N >= 3, "Point{N} does not have a z value!");
        self.coordinates[2]
    }

    pub fn w(&self) -> T {
        assert!(N >= 4, "Point{N} does not have a w value!");
        self.coordinates[3]
    }

    pub fn has_nan(&self) -> bool {
        for val in self.coordinates {
            if val.is_nan() {
                return false;
            }
        }
        return true;
    }

    pub fn length_sqr(&self) -> T {
        self.coordinates.iter().fold(T::zero(), |acc, x| acc + *x * *x)
    }

    pub fn length(&self) -> T {
        self.length_sqr().sqrt()
    }

    pub fn normalize(v: &Self) -> Self {
        let l = v.length();
        Self::init(v.coordinates.map(|x| x/l))
    }

    pub fn to_string(&self) -> String {
        format!(
            "({})",
            self.coordinates.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    pub fn min_component(&self) -> T {
        self.coordinates.iter().fold(self.coordinates[0], |acc, x| acc.min(*x))
    }

    pub fn max_component(&self) -> T {
        self.coordinates.iter().fold(self.coordinates[0], |acc, x| acc.max(*x))
    }

    pub fn min_dimension(&self) -> usize {
        let (idx, _) = self.coordinates.iter().enumerate().fold((0usize, self.coordinates[0]), |(min_idx, minimum), (idx, &value)| {
            if minimum < value {
                (idx, value)
            } else {
                (min_idx, minimum)
            }
        });

        idx
    }

    pub fn max_dimension(&self) -> usize {
        let (idx, _) = self.coordinates.iter().enumerate().fold((0usize, self.coordinates[0]), |(min_idx, maximum), (idx, &value)| {
            if maximum < value {
                (min_idx, maximum)
            } else {
                (idx, value)
            }
        });

        idx
    }

    pub fn min(v1: &Self, v2: &Self) -> Self {
        let mut new_min = v1.coordinates.clone();
        for i in 0..N {
            new_min[i] = new_min[i].min(v2[i]);
        }
        Self::init(
            new_min
        )
    }

    pub fn max(v1: &Self, v2: &Self) -> Self {
        let mut new_max = v1.coordinates.clone();
        for i in 0..N {
            new_max[i] = new_max[i].max(v2[i]);
        }
        Self::init(
            new_max
        )
    }

    pub fn permute(v: &Self, x: [usize; N]) -> Self {
        if x.iter().any(|&i| i >= N) {
            panic!("Index out of bounds for permutation");
        }

        Self::init(x.map(|i| v.coordinates[i]))
    }

    // pub fn distance(p1: &Self, p2: &Self) -> T {
    //     (*p1 - *p2).length()
    // }

    // pub fn distance_sqr(p1: &Self, p2: &Self) -> T {
    //     (*p1 - *p2).length_sqr()
    // }

    pub fn floor(&self) -> Self {
        Self {
            coordinates: self.coordinates.map(|x| x.floor())
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            coordinates: self.coordinates.map(|x| x.ceil())
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            coordinates: self.coordinates.map(|x| x.abs())
        }
    }
    
    pub fn equal(p1: &Self, p2: &Self) -> bool {
        p1.coordinates == p2.coordinates
    }

    pub fn dot(p1: &Self, p2: &Self) -> T {
        let mut ret = T::zero();
        for i in 0..N {
            ret = ret + p1[i]*p2[i];
        }

        ret
    }
}

impl<T, const N: usize> Mul<T> for Point<T, N>
    where
    T: Mul<Output = T> + Float + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            coordinates: self.coordinates.map(|x| x * rhs)
        }
    }
}

impl<T, const N: usize> Div<T> for Point<T, N>
    where
    T: Div<Output = T> + Float + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        assert!(rhs != T::zero(), "RHS cannot be zero for division!");
        Self {
            coordinates: self.coordinates.map(|x| x / rhs)
        }
    }
}

impl_operator!(Point<T, const N: usize>, Add, add, +, Point<T, const N: usize>);
impl_operator!(Point<T, const N: usize>, Sub, sub, -, Vector<T, const N: usize>);
impl_operator!(Point<T, const N: usize>, Mul, mul, *, Point<T, const N: usize>);
impl_operator_unary!(Point<T, const N: usize>, Neg, neg, -);
impl_operator_inplace!(Point<T, const N: usize>, AddAssign, add_assign, +=);
impl_operator_inplace!(Point<T, const N: usize>, SubAssign, sub_assign, -=);
impl_operator_inplace!(Point<T, const N: usize>, MulAssign, mul_assign, *=);

impl<T, const N: usize> Add<Vector<T, N>> for Point<T, N> 
where
    T: Add<Output = T> + Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug, 
{
        type Output = Self;

        fn add(self, rhs: Vector<T, N>) -> Self::Output {
            let mut coordinates = [T::zero(); N];
            for i in 0..N {
                coordinates[i] = self[i] + rhs[i];
            }
            Self {
                coordinates: coordinates
            }
        }
}

impl<T, const N: usize> Sub<Vector<T, N>> for Point<T, N> 
where
    T: Add<Output = T> + Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug, 
{
        type Output = Self;

        fn sub(self, rhs: Vector<T, N>) -> Self::Output {
            let mut coordinates = [T::zero(); N];
            for i in 0..N {
                coordinates[i] = self[i] - rhs[i];
            }
            Self {
                coordinates: coordinates
            }
        }
}

pub type Point3i = Point<i32, 3>;
pub type Point3f = Point<f32, 3>;
pub type Point3d = Point<f64, 3>;
pub type Point2i = Point<i32, 2>;
pub type Point2f = Point<f32, 2>;
pub type Point2d = Point<f64, 2>;
