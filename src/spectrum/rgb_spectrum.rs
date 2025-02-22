use std::ops::{Add, Div, Mul, Sub};

use crate::common::*;
use derive_more::{Index, IndexMut};

#[derive(Debug, Clone, Copy, Index, IndexMut)]
pub struct RGBSpectrum {
    #[index]
    #[index_mut]
    c: [f64; 3]
}

impl CoefficientSpectrum<3> for RGBSpectrum {
    fn c(&self) -> [f64; 3] { self.c }
    fn n_samples() -> usize { 3usize }

    fn init_one(v: f64) -> Self {
        Self {
            c: [v; 3]
        }
    }

    fn init_copy(s: &Self) -> Self {
        Self {
            c: s.c().clone()
        }
    }

    fn equals(&self, o: &Self) -> bool {
        for i in 0..3 {
            if self.c()[i] != o.c()[i] {
                return false;
            }
        }

        true
    }

    fn to_string(&self) -> String {
        format!("{:?}", self.c())
    }

    fn add(o1: &Self, o2: &Self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] + o2.c()[i];
        }

        Self {
            c
        }
    }

    fn sub(o1: &Self, o2: &Self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] - o2.c()[i];
        }

        Self {
            c
        }
    }

    fn div(o1: &Self, o2: &Self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] / o2.c()[i];
        }

        Self {
            c
        }
    }

    fn mul(o1: &Self, o2: &Self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] * o2.c()[i];
        }

        Self {
            c
        }
    }

    fn mul_one(o1: &Self, o2: f64) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] * o2;
        }

        Self {
            c
        }
    }

    fn div_one(o1: &Self, o2: f64) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = o1.c()[i] / o2;
        }

        Self {
            c
        }
    }

    fn add_assign(&mut self, s: &Self) {
        for i in 0..3 {
            self.c[i] += s.c[i];
        }
    }

    fn sub_assign(&mut self, s: &Self) {
        for i in 0..3 {
            self.c[i] -= s.c[i];
        }
    }

    fn mul_assign(&mut self, s: &Self) {
        for i in 0..3 {
            self.c[i] *= s.c[i];
        }
    }

    fn div_assign(&mut self, s: &Self) {
        for i in 0..3 {
            self.c[i] /= s.c[i];
        }
    }

    fn mul_assign_one(&mut self, o: f64) {
        for i in 0..3 {
            self.c[i] *= o;
        }
    }

    fn div_assign_one(&mut self, o: f64) {
        for i in 0..3 {
            self.c[i] /= o;
        }
    }

    fn sqrt(&self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = self.c[i].sqrt();
        }
        Self {
            c: c
        }
    }

    fn pow(&self, power: f64) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = self.c[i].powf(power);
        }
        Self {
            c: c
        }
    }
    
    fn exp(&self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = self.c[i].exp();
        }
        Self {
            c: c
        }
    }

    fn clamp(&self, low: f64, high: f64) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = self.c[i].clamp(low, high);
        }
        Self {
            c: c
        }
    }

    fn has_nan(&self) -> bool {
        for i in self.c {
            if i.is_nan() {
                return true;
            }
        }
        
        false
    }

    fn lerp(t: f64, a: &Self, b: &Self) -> Self {
        let mut c = [0f64; 3];
        for i in 0..3 {
            c[i] = (1.0 - t) * a.c[i] + t * b.c[i];
        }
        Self {
            c: c
        }
    }
}

impl RGBSpectrum {
    pub fn from_rgb(rgb: [f64; 3]) -> Self {
        let c = rgb.clone();
        Self {
            c
        }
    }

    pub fn from_xyz(xyz: [f64; 3]) -> Self {
        let mut rgb = [0.0; 3];

        rgb[0] =  3.240479*xyz[0] - 1.537150*xyz[1] - 0.498535*xyz[2];
        rgb[1] = -0.969256*xyz[0] + 1.875991*xyz[1] + 0.041556*xyz[2];
        rgb[2] =  0.055648*xyz[0] - 0.204043*xyz[1] + 1.057311*xyz[2];

        Self {
            c: rgb
        }
    }

    pub fn to_rgb(&self, rgb: &mut [f64; 3]) {
        for i in 0..3 {
            rgb[i] = self.c()[i];
        }
    }

    pub fn to_xyz(&self, xyz: &mut [f64; 3]) {
        xyz[0] = 0.412453 * self.c[0] + 0.357580 * self.c[1] + 0.180423 * self.c[2];
        xyz[1] = 0.212671 * self.c[0] + 0.715160 * self.c[1] + 0.072169 * self.c[2];
        xyz[2] = 0.019334 * self.c[0] + 0.119193 * self.c[1] + 0.950227 * self.c[2];
    }

    pub fn y(&self) -> f64 {
        let y_weight = [0.212671f64, 0.715160f64, 0.072169f64];

        y_weight[0] * self.c[0] + y_weight[1] * self.c[1] + y_weight[2] * self.c[2]
    }

    pub fn from_sampled(lambda: Vec<f64>, vals: Vec<f64>) -> Self {
        let mut xyz = [0f64; 3];
        for i in 0..N_CIE_SAMPLES {
            let val = Self::interpolate_spectrum_samples(&lambda, &vals, CIE_LAMBDA[i]);

            xyz[0] += val * CIE_X[i];
            xyz[1] += val * CIE_Y[i];
            xyz[2] += val * CIE_Z[i];
        }

        let scale = (CIE_LAMBDA[N_CIE_SAMPLES - 1] - CIE_LAMBDA[0]) / (CIE_Y_INTEGRAL * N_CIE_SAMPLES as f64);

        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;

        Self::from_xyz(xyz)
    }

    pub fn norm(&self) -> f64 {
        let mut ret = 0.0;
        for i in 0..3 {
            ret += self.c[i]*self.c[i];
        }

        ret.sqrt()
    }
}

impl Mul<f64> for RGBSpectrum {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] * rhs;
        c[1] = self.c[1] * rhs;
        c[2] = self.c[2] * rhs;
        Self {
            c
        }
    }
}

impl Div<f64> for RGBSpectrum {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] / rhs;
        c[1] = self.c[1] / rhs;
        c[2] = self.c[2] / rhs;
        Self {
            c
        }
    }
}

impl Mul<f32> for RGBSpectrum {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] * rhs as f64;
        c[1] = self.c[1] * rhs as f64;
        c[2] = self.c[2] * rhs as f64;
        Self {
            c
        }
    }
}

impl Div<f32> for RGBSpectrum {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] / rhs as f64;
        c[1] = self.c[1] / rhs as f64;
        c[2] = self.c[2] / rhs as f64;
        Self {
            c
        }
    }
}

impl Mul for RGBSpectrum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] * rhs[0];
        c[1] = self.c[1] * rhs[1];
        c[2] = self.c[2] * rhs[2];
        Self {
            c
        }
    }
}

impl Div for RGBSpectrum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] / rhs[0];
        c[1] = self.c[1] / rhs[1];
        c[2] = self.c[2] / rhs[2];
        Self {
            c
        }
    }
}

impl Add for RGBSpectrum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] + rhs[0];
        c[1] = self.c[1] + rhs[1];
        c[2] = self.c[2] + rhs[2];
        Self {
            c
        }
    }
}


impl Sub for RGBSpectrum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut c = [0f64; 3];
        c[0] = self.c[0] - rhs[0];
        c[1] = self.c[1] - rhs[1];
        c[2] = self.c[2] - rhs[2];
        Self {
            c
        }
    }
}