use num_traits::Float;
use std::fmt::Display;
use std::str::FromStr;
use std::ops::Mul;
use super::{point2::Point2, point3::Point3, vector2::Vector2, vector3::Vector3};

pub struct Bounds2<T> 
where T: Float + Copy
{
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

pub struct Bounds3<T> 
where T: Float + Copy
{
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

impl<T> Bounds2<T>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{
    pub fn new() -> Self {
        let min_num = T::neg_infinity();
        let max_num = T::infinity();
        Self {
            p_min: Point2::init(max_num, max_num),
            p_max: Point2::init(min_num, min_num),
        }
    }

    pub fn init(p_min: &Point2<T>, p_max: &Point2<T>) -> Self {
        Self {
            p_min: p_min.clone(),
            p_max: p_max.clone()
        }
    }

    pub fn init_one(p: &Point2<T>) -> Self {
        Self {
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(&self) -> Vector2<T> {
        self.p_max - self.p_min
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        d.x * d.y
    }

    pub fn max_extent(&self) -> i32 {
        let diag = self.diagonal();
        match diag.x > diag.y {
            true => 0,
            false => 1
        }
    }

    pub fn are_equal(b1: &Self, b2: &Self) -> bool {
        b1.p_min.x == b2.p_min.x && b1.p_min.y == b2.p_min.y && //b1.p_min.z == b2.p_min.z &&
        b1.p_max.x == b2.p_max.x && b1.p_max.y == b2.p_max.y// && b1.p_max.z == b2.p_max.z
    }

    pub fn lerp(&self, t: &Point2<T>) -> Point2<T> {
        let one = T::one();
        let x = self.p_min.x * (one - t.x) + t.x * self.p_max.x;
        let y = self.p_min.y * (one - t.y) + t.y * self.p_max.y;
        Point2::<T>::init(x, y)
    }

    pub fn offset(&self, p: &Point2<T>) -> Vector2<T> {
        let mut o = *p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x = o.x / self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y = o.y / self.p_max.y - self.p_min.y;
        }
        o
    }
}

impl<T> Bounds3<T>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug ,
    f32: Mul<T>
{
    pub fn new() -> Self {
        let min_num = T::neg_infinity();
        let max_num = T::infinity();
        Self {
            p_min: Point3::init(max_num, max_num, max_num),
            p_max: Point3::init(min_num, min_num, min_num),
        }
    }

    pub fn init(p_min: &Point3<T>, p_max: &Point3<T>) -> Self {
        Self {
            p_min: p_min.clone(),
            p_max: p_max.clone()
        }
    }

    pub fn init_one(p: &Point3<T>) -> Self {
        Self {
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(&self) -> Vector3<T> {
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        T::from(2f32).unwrap() * (d.x*d.y + d.x*d.z + d.y*d.z)
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn max_extent(&self) -> i32 {
        let d = self.diagonal();
        match d.x > d.y && d.x > d.z {
            true => 0,
            false => {
                match d.y > d.z {
                    true => 1,
                    false => 2
                }
            }
        }
    }

    pub fn corner(&self, corner: u8) -> Point3<T> {
        Point3 {
            x: if corner & 1 != 0 { self.p_max.x } else { self.p_min.x },
            y: if corner & 2 != 0 { self.p_max.y } else { self.p_min.y },
            z: if corner & 4 != 0 { self.p_max.z } else { self.p_min.z },
        }
    }

    pub fn are_equal(b1: &Self, b2: &Self) -> bool {
        b1.p_min.x == b2.p_min.x && b1.p_min.y == b2.p_min.y && b1.p_min.z == b2.p_min.z &&
        b1.p_max.x == b2.p_max.x && b1.p_max.y == b2.p_max.y && b1.p_max.z == b2.p_max.z
    }

    pub fn lerp(&self, t: &Point3<T>) -> Point3<T> {
        let one = T::one();
        let x = self.p_min.x * (one - t.x) + t.x * self.p_max.x;
        let y = self.p_min.y * (one - t.y) + t.y * self.p_max.y;
        let z = self.p_min.z * (one - t.z) + t.z * self.p_max.z;
        Point3::<T>::init(x, y, z)
    }

    pub fn offset(&self, p: &Point3<T>) -> Vector3<T> {
        let mut o = *p - self.p_min;
        if self.p_max.x > self.p_min.x {
            o.x = o.x / self.p_max.x - self.p_min.x;
        }
        if self.p_max.y > self.p_min.y {
            o.y = o.y / self.p_max.y - self.p_min.y;
        }
        if self.p_max.z > self.p_min.z {
            o.z = o.z / self.p_max.z - self.p_min.z;
        }
        o
    }

    pub fn bounding_sphere(&self, center: &mut Point3<T>, radius: &mut T) {
        *center = (self.p_min + self.p_max) / T::from(2f32).unwrap();
        *radius = match Self::inside(&center, self) {
            true => (*center - self.p_max).length(),
            false => T::zero()
        }
    }

    pub fn inside(p: &Point3<T>, b: &Self) -> bool {
        p.x >= b.p_min.x && p.x <= b.p_max.x &&
        p.y >= b.p_min.y && p.y <= b.p_max.y &&
        p.z >= b.p_min.z && p.z <= b.p_max.z
    }

    pub fn inside_exclusive(p: &Point3<T>, b: &Self) -> bool {
        p.x >= b.p_min.x && p.x < b.p_max.x &&
        p.y >= b.p_min.y && p.y < b.p_max.y &&
        p.z >= b.p_min.z && p.z < b.p_max.z
    }

    pub fn union_pt(b: &Self, p: &Point3<T>) -> Self {
        Self {
            p_min: Point3::<T>::min(&b.p_min, p),
            p_max: Point3::<T>::max(&b.p_max, p)
        }
    }

    pub fn union(b1: &Self, b2: &Self) -> Self {
        let p_min = Point3::<T>::min(&b1.p_min, &b2.p_min);
        let p_max = Point3::<T>::max(&b1.p_max, &b2.p_max);
        Self {
            p_min: p_min,
            p_max: p_max
        }
    }

    pub fn intersect(b1: &Self, b2: &Self) -> Self {
        let p_min = Point3::<T>::max(&b1.p_min, &b2.p_min);
        let p_max = Point3::<T>::min(&b1.p_max, &b2.p_max);
        Self {
            p_min: p_min,
            p_max: p_max
        }
    }

    pub fn overlaps(b1: &Self, b2: &Self) -> bool {
        let x = b1.p_max.x >= b2.p_min.x && b1.p_min.x <= b2.p_max.x;
        let y = b1.p_max.y >= b2.p_min.y && b1.p_min.y <= b2.p_max.y;
        let z = b1.p_max.z >= b2.p_min.z && b1.p_min.z <= b2.p_max.z;
        x && y && z
    }

    pub fn expand(b: &Self, delta: T) -> Self {
        Self {
            p_min: b.p_min + (-Vector3::<T>::init(delta, delta, delta)),
            p_max: b.p_max + (Vector3::<T>::init(delta, delta, delta)),
        }
    }
}

pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<f32>;
pub type Bounds3i = Bounds3<i32>;
pub type Bounds3f = Bounds3<f32>;