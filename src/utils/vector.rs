use crate::common::*;
use num_traits::Float;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, Neg};
use std::fmt::Display;
use std::str::FromStr;
use derive_more::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Index, IndexMut)]
pub struct Vector<T: Float + Copy, const N: usize> {
    #[index]
    #[index_mut]
    direction: [T; N]
}

impl<T, const N: usize> Vector<T, N>
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
        Self {
            direction: other.direction
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

    pub fn x(&self) -> T {
        self.direction[0]
    }

    pub fn y(&self) -> T {
        self.direction[1]
    }

    pub fn z(&self) -> T {
        assert!(N >= 3, "Vector{N} does not have a z value!");
        self.direction[2]
    }

    pub fn w(&self) -> T {
        assert!(N >= 4, "Vector{N} does not have a w value!");
        self.direction[3]
    }

    pub fn has_nan(&self) -> bool {
        for val in self.direction {
            if val.is_nan() {
                return false;
            }
        }
        return true;
    }

    pub fn length_sqr(&self) -> T {
        self.direction.iter().fold(T::zero(), |acc, x| acc + *x * *x)
    }

    pub fn length(&self) -> T {
        self.length_sqr().sqrt()
    }

    pub fn to_string(&self) -> String {
        format!(
            "({})",
            self.direction.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.direction.iter().enumerate()
            .fold(T::zero(), |dot_product, (idx, value)| dot_product + *value * v2[idx])
    }

    pub fn abs_dot(v1: &Self, v2: &Self) -> T {
        Self::dot(v1, v2).abs()
    }

    pub fn cross(v1: &Self, v2: &Self) -> Self {
        assert!(N == 3usize, "Only Vector3 can have cross product");
        let mut cross = [T::zero(); N];
        cross[0] = v1.y()*v2.z() - v2.y()*v1.z();
        cross[1] = v1.z()*v2.x() - v2.z()*v1.x();
        cross[2] = v1.x()*v2.y() - v2.y()*v1.x();

        Self::init(
            cross
        )
    }

    pub fn normalize(v: &Self) -> Self {
        let l = v.length();
        Self::init(v.direction.map(|x| x/l))
    }

    pub fn min_component(&self) -> T {
        self.direction.iter().fold(self.direction[0], |acc, x| acc.min(*x))
    }

    pub fn max_component(&self) -> T {
        self.direction.iter().fold(self.direction[0], |acc, x| acc.max(*x))

    }

    pub fn min_dimension(&self) -> usize {
        let (idx, _) = self.direction.iter().enumerate().fold((0usize, self.direction[0]), |(min_idx, minimum), (idx, &value)| {
            if minimum < value {
                (idx, value)
            } else {
                (min_idx, minimum)
            }
        });

        idx
    }

    pub fn max_dimension(&self) -> usize {
        let (idx, _) = self.direction.iter().enumerate().fold((0usize, self.direction[0]), |(min_idx, maximum), (idx, &value)| {
            if maximum < value {
                (min_idx, maximum)
            } else {
                (idx, value)
            }
        });

        idx
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
    
    pub fn equal(p1: &Self, p2: &Self) -> bool {
        p1.direction == p2.direction
    }

    pub fn abs(&self) -> Self {
        Self::init(self.direction.map(|x| x.abs()))
    }
}

impl_operator!(Vector<T, const N: usize>, Add, add, +, Vector<T, const N: usize>);
impl_operator!(Vector<T, const N: usize>, Sub, sub, -, Vector<T, const N: usize>);
impl_operator!(Vector<T, const N: usize>, Mul, mul, *, Vector<T, const N: usize>);
impl_operator_unary!(Vector<T, const N: usize>, Neg, neg, -);
impl_operator_inplace!(Vector<T, const N: usize>, AddAssign, add_assign, +=);
impl_operator_inplace!(Vector<T, const N: usize>, SubAssign, sub_assign, -=);
impl_operator_inplace!(Vector<T, const N: usize>, MulAssign, mul_assign, *=);

impl<T, const N: usize> Mul<T> for Vector<T, N>
    where
    T: Mul<Output = T> + Float + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            direction: self.direction.map(|x| x * rhs)
        }
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
    where
    T: Div<Output = T> + Float + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        assert!(rhs != T::zero(), "RHS cannot be zero for division!");
        Self {
            direction: self.direction.map(|x| x / rhs)
        }
    }
}

impl<T, const N: usize> Add<Point<T, N>> for Vector<T, N> 
    where
    T: Add<Output = T> + Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug, 
{
        type Output = Point<T, N>;

        fn add(self, rhs: Point<T, N>) -> Self::Output {
            let mut sum = [T::zero(); N];
            sum[0] = self[0] + rhs[0];
            sum[1] = self[1] + rhs[1];
            sum[2] = self[2] + rhs[2];
            Point::init(sum)
        }
}

impl<T, const N: usize> Sub<Point<T, N>> for Vector<T, N> 
    where
    T: Add<Output = T> + Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug, 
{
        type Output = Point<T, N>;

        fn sub(self, rhs: Point<T, N>) -> Self::Output {
            let mut sum = [T::zero(); N];
            sum[0] = self[0] - rhs[0];
            sum[1] = self[1] - rhs[1];
            sum[2] = self[2] - rhs[2];
            Point::init(sum)
        }
}

pub fn coordinate_system<T>(v1: &Vector<T, 3usize>, v2: &mut Vector<T, 3usize>, v3: &mut Vector<T, 3usize>)
where T: Float + Copy + Display + FromStr,
<T as FromStr>::Err: std::fmt::Debug  
{
    if v1.x().abs() > v1.y().abs() {
        *v2 = Vector::<T, 3usize>::init([-v1.z(), T::zero(), v1.x()]);
        *v2 = Vector::<T, 3usize>::normalize(v2);
    } else {
        *v2 = Vector::<T, 3usize>::init([T::zero(), v1.z(), -v1.y()]);
        *v2 = Vector::<T, 3usize>::normalize(v2);
    }

    *v3 = Vector::<T, 3usize>::cross(v1, v2);
}

pub type Vector3i = Vector<i32, 3usize>;
pub type Vector3f = Vector<f32, 3usize>;
pub type Vector3d = Vector<f64, 3usize>;
pub type Vector2i = Vector<i32, 2usize>;
pub type Vector2f = Vector<f32, 2usize>;
pub type Vector2d = Vector<f64, 2usize>;
