use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct Frame {

}

impl Frame {
    pub fn cos_theta(w: &Vector3f) -> f32 {
        w.z()
    }

    pub fn cos2_theta(w: &Vector3f) -> f32 {
        w.z() * w.z()
    }

    pub fn abs_cos_theta(w: &Vector3f) -> f32 {
        w.z().abs()
    }

    pub fn sin2_theta(w: &Vector3f) -> f32 {
        0f32.max(1.0 - Self::cos2_theta(w))
    }

    pub fn sin_theta(w: &Vector3f) -> f32 {
        Self::sin2_theta(w).sqrt()
    }

    pub fn tan_theta(w: &Vector3f) -> f32 {
        Self::sin_theta(w) / Self::cos_theta(w)
    }

    pub fn tan2_theta(w: &Vector3f) -> f32 {
        Self::sin2_theta(w) / Self::cos2_theta(w)
    }

    pub fn cos_phi(w: &Vector3f) -> f32 {
        let sin = Self::sin_theta(w);

        if sin == 0.0 {
            0.0
        } else {
            (w.x() / sin).clamp(-1.0, 1.0)
        }
    }

    pub fn sin_phi(w: &Vector3f) -> f32 {
        let sin = Self::sin_theta(w);

        if sin == 0.0 {
            0.0
        } else {
            (w.y() / sin).clamp(-1.0, 1.0)
        }
    }

    pub fn cos2_phi(w: &Vector3f) -> f32 {
        Self::cos_phi(w) * Self::cos_phi(w)
    }

    pub fn sin2_phi(w: &Vector3f) -> f32 {
        Self::sin_phi(w) * Self::sin_phi(w)
    }

    pub fn cos_d_phi(wa: &Vector3f, wb: &Vector3f) -> f32 {
        let num = wa.x() * wb.x() + wa.y() * wb.y();
        let denom = (wa.x()*wa.x() + wa.y()*wa.y()) * (wb.x()*wb.x() + wb.y()*wb.y());

        (num/denom).clamp(-1.0, 1.0)
    }
}