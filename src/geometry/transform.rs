use crate::common::{Bounds3f, Normal3f, SurfaceInteraction, Vector3f, Ray, RayDifferential};

use super::{matrix::Matrix4x4, normal::Normal, point::{Point, Point3f}, vector::Vector};
use std::fmt::Display;
use std::str::FromStr;
use std::ops::Mul;
use num_traits::Float;
use derive_more::{Index, IndexMut};

#[derive(Debug, Clone, Index, IndexMut)]
pub struct Transform{
    #[index]
    #[index_mut]
    pub m: Matrix4x4,
    pub m_inv: Matrix4x4
}

impl Transform {
    pub fn new() -> Self {
        Self {
            m: Matrix4x4::identity(),
            m_inv: Matrix4x4::identity()
        }
    }

    pub fn init(m: &Vec<Vec<f32>>) -> Self {
        assert!(m.len() == 4 && m[0].len() == 4, "Transform matrix needs to be 4x4!");
        let mat = Matrix4x4::init(
            m[0][0], m[0][1], m[0][2], m[0][3],
            m[1][0], m[1][1], m[1][2], m[1][3],
            m[2][0], m[2][1], m[2][2], m[2][3],
            m[3][0], m[3][1], m[3][2], m[3][3]
        );

        Self {
            m_inv: mat.inverse(),
            m: mat
        }
    }

    pub fn init_mat(m: &Matrix4x4) -> Self {
        Self {
            m_inv: m.inverse(),
            m: m.clone()
        }
    }

    pub fn translate(t: &Vector3f) -> Self {
        let mat = Matrix4x4::init(1.0, 0.0, 0.0, t.x(), 
            0.0, 1.0, 0.0, t.y(), 
            0.0, 0.0, 1.0, t.z(), 
            0.0, 0.0, 0.0, 1.0);

        Self {
            m_inv: mat.inverse(),
            m: mat
        }
    }

    pub fn scale(t: &Vector3f) -> Self {
        let t= t.abs();
        let mat = Matrix4x4::init(t.x(), 0.0, 0.0, 0.0, 
            0.0, t.y(), 0.0, 0.0, 
            0.0, 0.0, t.z(), 0.0, 
            0.0, 0.0, 0.0, 1.0);

        Self {
            m_inv: mat.inverse(),
            m: mat
        }
    }

    pub fn rotate(theta: f32, axis: &Vector3f) -> Self {
        let a = Vector3f::normalize(axis);
        let cos_theta: f32 = theta.to_radians().cos();
        let sin_theta: f32 = theta.to_radians().sin();

        let mut mat: Matrix4x4 = Matrix4x4::identity();

        mat[0][0] = a.x()*a.x() + (1.0 - a.x()*a.x()) * cos_theta;
        mat[0][1] = a.x()*a.y()*(1.0 - cos_theta) - a.z()*sin_theta;
        mat[0][2] = a.x()*a.z()*(1.0 - cos_theta) + a.y()*sin_theta;
        mat[0][3] = 0.0;

        mat[1][0] = a.x()*a.y()*(1.0 - cos_theta) + a.z()*sin_theta;
        mat[1][1] = a.y()*a.y() + (1.0 - a.y()*a.y())*cos_theta;
        mat[1][2] = a.y()*a.z()*(1.0 - cos_theta) - a.x()*sin_theta;
        mat[1][3] = 0.0;

        mat[2][0] = a.x()*a.z()*(1.0 - cos_theta) - a.y()*sin_theta;
        mat[2][1] = a.y()*a.z()*(1.0 - cos_theta) + a.x()*sin_theta;
        mat[2][2] = a.z()*a.z() + (1.0 - a.z()*a.z())*cos_theta;
        mat[2][3] = 0.0;

        Self {
            m_inv: mat.inverse(),
            m: mat
        }
    }

    pub fn look_at(eye: &Point3f, look: &Point3f, up: &Vector3f) -> Self {
        let mut m = Matrix4x4::identity();

        m[0][3] = eye.x();
        m[1][3] = eye.y();
        m[2][3] = eye.z();
        m[3][3] = 1.0;

        let dir = Vector3f::normalize(&((*look) - (*eye)));
        let right = Vector3f::normalize(
            &Vector3f::cross(
                &Vector3f::normalize(up), 
                &dir
            ));
        let new_up = Vector3f::cross(&dir, &right);

        m[0][0] = right.x();
        m[1][0] = right.y();
        m[2][0] = right.z();
        m[3][0] = 0.0;
        m[0][1] = new_up.x();
        m[1][1] = new_up.y();
        m[2][1] = new_up.z();
        m[3][1] = 0.0;
        m[0][2] = dir.x();
        m[1][2] = dir.y();
        m[2][2] = dir.z();
        m[3][2] = 0.0;

        Self{
            m_inv: m.inverse(),
            m: m
        }
    }

    pub fn is_identity(&self) -> bool{
        for i in 0..4 {
            for j in 0..4 {
                match i == j {
                    true => if self.m[i][j] != 1f32 {return false;}
                    false => if self.m[i][j] != 0f32 {return false;}
                }
            }
        }

        true
    }

    pub fn get_matrix(&self) -> Matrix4x4 {
        self.m.clone()
    }

    pub fn get_inv_matrix(&self) -> Matrix4x4 {
        self.m_inv.clone()
    }

    pub fn inverse(&self) -> Self {
        Self {
            m: self.m_inv.clone(),
            m_inv: self.m.clone()
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            m: self.m.transpose(),
            m_inv: self.m_inv.transpose()
        }
    }

    pub fn equal(t1: &Self, t2: &Self) -> bool {
        Matrix4x4::equal(&t1.m, &t2.m)
    }

    // If the transform changes teh handedness of coordinate system
    pub fn swaps_handedness(&self) -> bool {
        let det = self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) -
        self.m[0][1] * (self.m[1][0] * self.m[2][2] - self.m[1][2] * self.m[2][0]) +
        self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0]);

        det < 0.0
    }

    pub fn to_string(&self) -> String {
        format!(
            "[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]\n[{}, {}, {}, {}]",
            self.m[0][0], self.m[0][1], self.m[0][2], self.m[0][3],
            self.m[1][0], self.m[1][1], self.m[1][2], self.m[1][3],
            self.m[2][0], self.m[2][1], self.m[2][2], self.m[2][3],
            self.m[3][0], self.m[3][1], self.m[3][2], self.m[3][3]
        )
    }
}

impl<T, const N: usize> Mul<Point<T, N>> for &Transform 
    where
    T: Mul<Output = T> + Float + Copy + FromStr + Display, 
    <T as FromStr>::Err: std::fmt::Debug 
{
    type Output = Point<T, N>;

    fn mul(self, p: Point<T, N>) -> Self::Output {
        let x = p.x();
        let y = p.y();
        let z = p.z();

        let xp = T::from(self.m[0][0]).unwrap()*x + T::from(self.m[0][1]).unwrap()*y + T::from(self.m[0][2]).unwrap()*z + T::from(self.m[0][3]).unwrap();
        let yp = T::from(self.m[1][0]).unwrap()*x + T::from(self.m[1][1]).unwrap()*y + T::from(self.m[1][2]).unwrap()*z + T::from(self.m[1][3]).unwrap();
        let zp = T::from(self.m[2][0]).unwrap()*x + T::from(self.m[2][1]).unwrap()*y + T::from(self.m[2][2]).unwrap()*z + T::from(self.m[2][3]).unwrap();
        let wp = T::from(self.m[3][0]).unwrap()*x + T::from(self.m[3][1]).unwrap()*y + T::from(self.m[3][2]).unwrap()*z + T::from(self.m[3][3]).unwrap();
        let mut direction = [T::zero(); N];
        direction[0] = xp;
        direction[1] = yp;
        direction[2] = zp;
        if wp == T::from(1.0).unwrap() {
            return Point::<T, N>::init(direction);
        }
        direction[0] = direction[0] / wp;
        direction[1] = direction[1] / wp;
        direction[2] = direction[2] / wp;

        return Point::<T, N>::init(direction);
    }
}

impl<T, const N: usize> Mul<Vector<T, N>> for &Transform 
    where
    T: Mul<Output = T> + Float + Copy + FromStr + Display, 
    <T as FromStr>::Err: std::fmt::Debug 
{
    type Output = Vector<T, N>;

    fn mul(self, p: Vector<T, N>) -> Self::Output {
        let x = p.x();
        let y = p.y();
        let z = p.z();

        let xp = T::from(self.m[0][0]).unwrap()*x + T::from(self.m[0][1]).unwrap()*y + T::from(self.m[0][2]).unwrap()*z;
        let yp = T::from(self.m[1][0]).unwrap()*x + T::from(self.m[1][1]).unwrap()*y + T::from(self.m[1][2]).unwrap()*z;
        let zp = T::from(self.m[2][0]).unwrap()*x + T::from(self.m[2][1]).unwrap()*y + T::from(self.m[2][2]).unwrap()*z;
        
        let mut direction = [T::zero(); N];
        direction[0] = xp;
        direction[1] = yp;
        direction[2] = zp;
        Vector::<T, N>::init(direction)
    }
}

// For normals, so that they are perpendicular to the surface, to apply the Transformation T, we multiply with T_inverse_transpose
impl<T, const N: usize> Mul<Normal<T, N>> for &Transform 
    where T: Float + Copy + Display + FromStr,
    <T as FromStr>::Err: std::fmt::Debug  
{
    type Output = Normal<T, N>;

    fn mul(self, p: Normal<T, N>) -> Self::Output {
        let x = p.x();
        let y = p.y();
        let z = p.z();

        let xp = T::from(self.m_inv[0][0]).unwrap()*x + T::from(self.m_inv[1][0]).unwrap()*y + T::from(self.m_inv[2][0]).unwrap()*z;
        let yp = T::from(self.m_inv[0][1]).unwrap()*x + T::from(self.m_inv[1][1]).unwrap()*y + T::from(self.m_inv[2][1]).unwrap()*z;
        let zp = T::from(self.m_inv[0][2]).unwrap()*x + T::from(self.m_inv[1][2]).unwrap()*y + T::from(self.m_inv[2][2]).unwrap()*z;
        
        let mut direction = [T::zero(); N];
        direction[0] = xp;
        direction[1] = yp;
        direction[2] = zp;
        Normal::<T, N>::init(direction)
    }
}

impl Mul<Bounds3f> for &Transform {
    type Output = Bounds3f;

    fn mul(self, b: Bounds3f) -> Self::Output {
        let mut ret = Bounds3f::new();
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_min.x(), b.p_min.y(), b.p_min.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_max.x(), b.p_min.y(), b.p_min.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_min.x(), b.p_max.y(), b.p_min.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_min.x(), b.p_min.y(), b.p_max.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_min.x(), b.p_max.y(), b.p_max.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_max.x(), b.p_max.y(), b.p_min.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_max.x(), b.p_min.y(), b.p_max.z()])));
        ret = Bounds3f::union_pt(&ret, &(self * Point3f::init([b.p_max.x(), b.p_max.y(), b.p_max.z()])));
        ret
    }
}

impl Mul<&Ray> for &Transform {
    type Output = Ray;

    fn mul(self, b: &Ray) -> Self::Output {
        let mut ret = Ray::new();
        ret.o = self * b.o;
        ret.d = self * b.d;
        ret.t_min = b.t_min;
        ret.t_max = b.t_max;
        ret.medium = b.medium.clone();

        ret
    }
}

impl Mul<&RayDifferential> for &Transform {
    type Output = RayDifferential;

    fn mul(self, b: &RayDifferential) -> Self::Output {
        let mut ret = RayDifferential::new();
        ret.o = self * b.o;
        ret.d = self * b.d;
        ret.rx_o = self * b.rx_o;
        ret.rx_d = self * b.rx_d;
        ret.ry_o = self * b.ry_o;
        ret.ry_d = self * b.ry_d;
        ret.has_differential = b.has_differential;

        ret.t_min = b.t_min;
        ret.t_max = b.t_max;
        ret.medium = b.medium.clone();

        ret
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        Transform {
            m: Matrix4x4::mul(&self.m, &rhs.m),
            m_inv: Matrix4x4::mul(&rhs.m, &self.m)
        }
    }
}

impl Mul<&SurfaceInteraction> for &Transform {
    type Output = SurfaceInteraction;

    fn mul(self, s: &SurfaceInteraction) -> Self::Output {
        let mut ret = SurfaceInteraction::new();

        ret.p = self * s.p;
        ret.n = Normal3f::normalize(&(self * s.n));
        ret.wo = self * s.wo;
        ret.t = s.t;
        ret.medium_interface = s.medium_interface.clone();
        ret.uv = s.uv;
        ret.dpdu = self * s.dpdu;
        ret.dpdv = self * s.dpdv;
        ret.dndu = self * s.dndu;
        ret.dndv = self * s.dndv;
        ret.shape = s.shape.clone();
        ret.shading.n = Normal3f::normalize(&(self * s.shading.n));
        ret.shading.dpdu = self * s.shading.dpdu;
        ret.shading.dpdv = self * s.shading.dpdv;
        ret.shading.dndu = self * s.shading.dndu;
        ret.shading.dndv = self * s.shading.dndv;        
        ret.dudx = s.dudx;
        ret.dvdx = s.dvdx;
        ret.dudy = s.dudy;
        ret.dvdy = s.dvdy;
        ret.dpdx = self * s.dpdx;
        ret.dpdy = self * s.dpdy;

        // TODO - ADD BSDF, BSSRDF, PRIMITIVE copying as well

        let temp = Vector3f::init([ret.n.x(), ret.n.y(), ret.n.z()]);
        ret.shading.n = Normal3f::faceforward(&ret.shading.n, &temp);

        ret
    }
}