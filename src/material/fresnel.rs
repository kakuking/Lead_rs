use crate::common::*;

pub enum Fresnel {
    Conductor(FresnelConductor),
    Dielectric(FresnelDielectric),
    NoOp(FresnelNoOp)
}

impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f32) -> Spectrum {
        match self{
            Fresnel::Conductor(s) => s.evaluate(cos_theta_i),
            Fresnel::Dielectric(s) => s.evaluate(cos_theta_i),
            Fresnel::NoOp(s) => s.evaluate(cos_theta_i),
        }
    }
}

pub struct FresnelConductor {
    pub eta_i: Spectrum,
    pub eta_t: Spectrum,
    pub k: Spectrum,
}

impl FresnelConductor {
    pub fn evaluate(&self, cos_theta_i: f32) -> Spectrum {
        let fr = fr_conductor(cos_theta_i, self.eta_i, self.eta_t, self.k);

        Spectrum::init_one(fr as f64)
    }
}

pub struct FresnelDielectric {
    pub eta_i: Spectrum,
    pub eta_t: Spectrum,
}

impl FresnelDielectric {
    pub fn evaluate(&self, cos_theta_i: f32) -> Spectrum {
        RGBSpectrum::init_one(fr_dielectric(cos_theta_i, self.eta_i, self.eta_t) as f64)
    }
}

pub struct FresnelNoOp {

}

impl FresnelNoOp {
    pub fn evaluate(&self, _: f32) -> Spectrum {
        Spectrum::init_one(1.0)
    }
}

pub fn fr_dielectric(cos_theta_i: f32, eta_i: Spectrum, eta_t: Spectrum) -> f32 {
    let mut cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
    let entering = cos_theta_i > 0.0;

    let mut eta_i = eta_i; let mut eta_t = eta_t;
    if !entering {
        let t = eta_i;
        eta_i = eta_t;
        eta_t = t;
        cos_theta_i = cos_theta_i.abs();
    }

    let sin_theta_i = 0f32.max(1.0 - cos_theta_i*cos_theta_i).sqrt();
    let sin_theta_t = eta_i / eta_t * sin_theta_i;

    if sin_theta_t.y() >= 1.0 {
        return 1.0;
    }

    let cos_theta_t = 0f32.max((Spectrum::init_one(1.0) - sin_theta_t*sin_theta_t).norm() as f32).sqrt();

    let r_parallel = (eta_t*cos_theta_i - eta_i*cos_theta_t) / (eta_t*cos_theta_i + eta_i*cos_theta_t);
    let r_perp = (eta_i*cos_theta_i - eta_t*cos_theta_t) / (eta_i*cos_theta_i + eta_t*cos_theta_t);

    ((r_parallel*r_parallel + r_perp*r_perp).norm() * 0.5) as f32
}

pub fn fr_conductor(cos_theta_i: f32, eta_i: Spectrum, eta_t: Spectrum, k: Spectrum) -> f32 {
    let cos_theta_i = cos_theta_i.clamp(-1.0, 1.0);
    let eta_i = eta_i[0] as f32;
    let eta_t = eta_t[1] as f32;
    let k = k[0] as f32;
    let eta = eta_t / eta_i;
    let eta_k = k / eta_i;

    let cos_theta_i2 = cos_theta_i * cos_theta_i;
    let sin_theta_i2 = 1.0 - cos_theta_i2;
    let eta_2 = eta * eta;
    let eta_k2 = eta_k * eta_k;

    let t0 = eta_2 - eta_k2 - sin_theta_i2;
    let a2_plus_b2 = (t0*t0 + 4.0*eta_2*eta_k2).sqrt();
    let t1 = a2_plus_b2 + cos_theta_i2;
    let a = (0.5 * (a2_plus_b2 + t0)).sqrt();
    let t2 = 2.0 * cos_theta_i * a;
    let r_s = (t1 - t2) / (t1 + t2);

    let t3 = cos_theta_i2 * a2_plus_b2 + sin_theta_i2*sin_theta_i2;
    let t4 = t2 * sin_theta_i2;
    let r_p = r_s * (t3 - t4) / (t3 + t4);

    0.5 * (r_p + r_s)
}