use num_traits::Float;
use std::fmt::Display;
use std::str::FromStr;
use std::ops::Mul;

use super::{point::Point, ray::Ray, vector::Vector};

pub struct Bounds2<T> 
where T: Float + Copy
{
    pub p_min: Point<T, 2usize>,
    pub p_max: Point<T, 2usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds3<T> 
where T: Float + Copy
{
    pub p_min: Point<T, 3usize>,
    pub p_max: Point<T, 3usize>,
}

impl<T> Bounds2<T>
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug 
{
    pub fn new() -> Self {
        let min_num = T::neg_infinity();
        let max_num = T::infinity();
        Self {
            p_min: Point::init([max_num, max_num]),
            p_max: Point::init([min_num, min_num]),
        }
    }

    pub fn init(p_min: &Point<T, 2usize>, p_max: &Point<T, 2usize>) -> Self {
        Self {
            p_min: p_min.clone(),
            p_max: p_max.clone()
        }
    }

    pub fn init_one(p: &Point<T, 2usize>) -> Self {
        Self {
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(&self) -> Vector<T, 2usize> {
        self.p_max - self.p_min
    }

    pub fn area(&self) -> T {
        let d = self.diagonal();
        (d.x() * d.y()).abs()
    }

    pub fn max_extent(&self) -> i32 {
        let diag = self.diagonal();
        match diag.x() > diag.y() {
            true => 0,
            false => 1
        }
    }

    pub fn are_equal(b1: &Self, b2: &Self) -> bool {
        b1.p_min.x() == b2.p_min.x() && b1.p_min.y() == b2.p_min.y() && //b1.p_min.z() == b2.p_min.z() &&
        b1.p_max.x() == b2.p_max.x() && b1.p_max.y() == b2.p_max.y()// && b1.p_max.z() == b2.p_max.z()
    }

    pub fn lerp(&self, t: &Point<T, 2usize>) -> Point<T, 2usize> {
        let one = T::one();
        let x = self.p_min.x() * (one - t.x()) + t.x() * self.p_max.x();
        let y = self.p_min.y() * (one - t.y()) + t.y() * self.p_max.y();
        Point::<T, 2usize>::init([x, y])
    }

    pub fn offset(&self, p: &Point<T, 2usize>) -> Vector<T, 2usize> {
        let mut o = *p - self.p_min;
        if self.p_max.x() > self.p_min.x() {
            o[0] = o.x() / self.p_max.x() - self.p_min.x();
        }
        if self.p_max.y() > self.p_min.y() {
            o[1] = o.y() / self.p_max.y() - self.p_min.y();
        }
        o
    }

    pub fn to_string(&self) -> String {
        format!(
            "[\n  p_min: {}\n  p_max: {}\n]",
            self.p_min.to_string(),
            self.p_max.to_string()
        )
    }

    pub fn get(&self, i: usize) -> Point<T, 2usize> {
        match i {
            0usize => {return  self.p_min;}
            1usize => {return  self.p_max;}
            _ => {panic!("Invalid index for bounding box!")}
        }
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
            p_min: Point::<T, 3usize>::init([max_num, max_num, max_num]),
            p_max: Point::<T, 3usize>::init([min_num, min_num, min_num]),
        }
    }

    pub fn init(p_min: &Point<T, 3usize>, p_max: &Point<T, 3usize>) -> Self {
        Self {
            p_min: p_min.clone(),
            p_max: p_max.clone()
        }
    }

    pub fn init_one(p: &Point<T, 3usize>) -> Self {
        Self {
            p_min: p.clone(),
            p_max: p.clone()
        }
    }

    pub fn diagonal(&self) -> Vector<T, 3usize> {
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> T {
        let d = self.diagonal();
        T::from(2f32).unwrap() * (d.x()*d.y() + d.x()*d.z() + d.y()*d.z())
    }

    pub fn volume(&self) -> T {
        let d = self.diagonal();
        d.x() * d.y() * d.z()
    }

    pub fn max_extent(&self) -> i32 {
        let d = self.diagonal();
        match d.x() > d.y() && d.x() > d.z() {
            true => 0,
            false => {
                match d.y() > d.z() {
                    true => 1,
                    false => 2
                }
            }
        }
    }

    pub fn corner(&self, corner: u8) -> Point<T, 3usize> {
        let x = if corner & 1 != 0 { self.p_max.x() } else { self.p_min.x() };
        let y = if corner & 2 != 0 { self.p_max.y() } else { self.p_min.y() };
        let z = if corner & 4 != 0 { self.p_max.z() } else { self.p_min.z() };
        Point::<T, 3usize>::init([x, y, z])
    }

    pub fn are_equal(b1: &Self, b2: &Self) -> bool {
        b1.p_min.x() == b2.p_min.x() && b1.p_min.y() == b2.p_min.y() && b1.p_min.z() == b2.p_min.z() &&
        b1.p_max.x() == b2.p_max.x() && b1.p_max.y() == b2.p_max.y() && b1.p_max.z() == b2.p_max.z()
    }

    pub fn lerp(&self, t: &Point<T, 3usize>) -> Point<T, 3usize> {
        let one = T::one();
        let x = self.p_min.x() * (one - t.x()) + t.x() * self.p_max.x();
        let y = self.p_min.y() * (one - t.y()) + t.y() * self.p_max.y();
        let z = self.p_min.z() * (one - t.z()) + t.z() * self.p_max.z();
        Point::<T, 3usize>::init([x, y, z])
    }

    pub fn offset(&self, p: &Point<T, 3usize>) -> Vector<T, 3usize> {
        let mut o = *p - self.p_min;
        if self.p_max.x() > self.p_min.x() {
            o[0] = o.x() / self.p_max.x() - self.p_min.x();
        }
        if self.p_max.y() > self.p_min.y() {
            o[1] = o.y() / self.p_max.y() - self.p_min.y();
        }
        if self.p_max.z() > self.p_min.z() {
            o[2] = o.z() / self.p_max.z() - self.p_min.z();
        }
        o
    }

    pub fn bounding_sphere(&self, center: &mut Point<T, 3usize>, radius: &mut T) {
        *center = (self.p_min + self.p_max) / T::from(2f32).unwrap();
        *radius = match Self::inside(&center, self) {
            true => (*center - self.p_max).length(),
            false => T::zero()
        }
    }

    pub fn inside(p: &Point<T, 3usize>, b: &Self) -> bool {
        p.x() >= b.p_min.x() && p.x() <= b.p_max.x() &&
        p.y() >= b.p_min.y() && p.y() <= b.p_max.y() &&
        p.z() >= b.p_min.z() && p.z() <= b.p_max.z()
    }

    pub fn inside_exclusive(p: &Point<T, 3usize>, b: &Self) -> bool {
        p.x() >= b.p_min.x() && p.x() < b.p_max.x() &&
        p.y() >= b.p_min.y() && p.y() < b.p_max.y() &&
        p.z() >= b.p_min.z() && p.z() < b.p_max.z()
    }

    pub fn union_pt(b: &Self, p: &Point<T, 3usize>) -> Self {
        Self {
            p_min: Point::<T, 3usize>::min(&b.p_min, p),
            p_max: Point::<T, 3usize>::max(&b.p_max, p)
        }
    }

    pub fn union(b1: &Self, b2: &Self) -> Self {
        let p_min = Point::<T, 3usize>::min(&b1.p_min, &b2.p_min);
        let p_max = Point::<T, 3usize>::max(&b1.p_max, &b2.p_max);
        Self {
            p_min: p_min,
            p_max: p_max
        }
    }

    pub fn intersect(b1: &Self, b2: &Self) -> Self {
        let p_min = Point::<T, 3usize>::max(&b1.p_min, &b2.p_min);
        let p_max = Point::<T, 3usize>::min(&b1.p_max, &b2.p_max);
        Self {
            p_min: p_min,
            p_max: p_max
        }
    }

    pub fn overlaps(b1: &Self, b2: &Self) -> bool {
        let x = b1.p_max.x() >= b2.p_min.x() && b1.p_min.x() <= b2.p_max.x();
        let y = b1.p_max.y() >= b2.p_min.y() && b1.p_min.y() <= b2.p_max.y();
        let z = b1.p_max.z() >= b2.p_min.z() && b1.p_min.z() <= b2.p_max.z();
        x && y && z
    }

    pub fn expand(b: &Self, delta: T) -> Self {
        Self {
            p_min: b.p_min + (-Vector::<T, 3usize>::init([delta, delta, delta])),
            p_max: b.p_max + (Vector::<T, 3usize>::init([delta, delta, delta])),
        }
    }

    pub fn intersect_p(&self, ray: &Ray, hit_0: &mut f32, hit_1: &mut f32) -> bool {
        let mut t_0: f32 = 0.0;
        let mut t_1: f32 = ray.t_max;

        for i in 0..=3 {
            let inv_ray_dir = 1.0 / ray.d[i];
            let mut t_near = (self.p_min[i] - T::from(ray.o[i]).unwrap()) * T::from(inv_ray_dir).unwrap();
            let mut t_far = (self.p_max[i] - T::from(ray.o[i]).unwrap()) * T::from(inv_ray_dir).unwrap();

            if t_near > t_far {
                let temp = t_near;
                t_near = t_far;
                t_far = temp;

                t_0 = if !t_near.is_nan() && t_near > T::from(t_0).unwrap() { t_near.to_f32().unwrap() } else { t_0 };
                t_1 = if !t_far.is_nan() && t_far < T::from(t_1).unwrap() { t_far.to_f32().unwrap() } else { t_1 };

                if t_0 > t_1 {
                    return false;
                }
            }
        }

        *hit_0 = t_0;
        *hit_1 = t_1;

        true
    }

    pub fn intersect_inv_p(&self, ray: &Ray, inv_dir: &Vector<T, 3usize>, dir_is_neg: [i32; 3]) -> bool {
        let bounds = self;  // `self` is a reference to Bounds3f
        
        // Check for ray intersection against slabs in X, Y, and Z directions
        let x_min = bounds.get(dir_is_neg[0] as usize).x();
        let x_max = bounds.get((1 - dir_is_neg[0]) as usize).x();
        let y_min = bounds.get(dir_is_neg[1] as usize).y();
        let y_max = bounds.get((1 - dir_is_neg[1]) as usize).y();
    
        let mut t_min = (x_min - T::from(ray.o.x()).unwrap()) * inv_dir.x();
        let mut t_max = (x_max - T::from(ray.o.x()).unwrap()) * inv_dir.x();
        let ty_min = (y_min - T::from(ray.o.y()).unwrap()) * inv_dir.y();
        let ty_max = (y_max - T::from(ray.o.y()).unwrap()) * inv_dir.y();
        
        // Update tMax and tyMax to ensure robust bounds intersection
        if t_min > ty_max || ty_min > t_max {
            return false;
        }
        if ty_min > t_min {
            t_min = ty_min;
        }
        if ty_max < t_max {
            t_max = ty_max;
        }
        
        // Check for ray intersection against slabs in the Z direction
        let z_min = bounds.get(dir_is_neg[2] as usize).z();
        let z_max = bounds.get((1 - dir_is_neg[2]) as usize).z();
        
        let tz_min = (z_min - T::from(ray.o.z()).unwrap()) * inv_dir.z();
        let tz_max = (z_max - T::from(ray.o.z()).unwrap()) * inv_dir.z();
        
        // Update tzMax to ensure robust bounds intersection
        if t_min > tz_max || tz_min > t_max {
            return false;
        }
        if tz_min > t_min {
            t_min = tz_min;
        }
        if tz_max < t_max {
            t_max = tz_max;
        }
    
        return t_min < T::from(ray.t_max).unwrap() && t_max > T::zero();
    }
    

    pub fn to_string(&self) -> String {
        format!(
            "[\n  p_min: {}\n  p_max: {}\n]",
            self.p_min.to_string(),
            self.p_max.to_string()
        )
    }

    
    pub fn get(&self, i: usize) -> Point<T, 3usize> {
        match i {
            0usize => {return  self.p_min;}
            1usize => {return  self.p_max;}
            _ => {panic!("Invalid index for bounding box!")}
        }
    }
}

pub type Bounds2f = Bounds2<f32>;
pub type Bounds3f = Bounds3<f32>;