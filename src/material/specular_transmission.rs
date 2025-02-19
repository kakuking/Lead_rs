use crate::common::*;

pub struct SpecularTransmission {
    t: Spectrum,
    fresnel: FresnelDielectric,
    eta_a: f32, eta_b: f32,
    mode: TransportMode
}

impl BxDF for SpecularTransmission {
    fn bxdf_type(&self) -> u32 {
        BxDFType::BSDFTtransmission.to_u32() | BxDFType::BSDFSpecular.to_u32()
    }

    fn sample_f(&self, wo: &Vector3f, wi: &mut Vector3f, _sample: &Point2f, pdf: &mut f32, _sampled_type: Option<BxDFType>) -> Spectrum {
        let entering = Frame::cos_theta(wo) > 0.0;
        let eta_i = if entering { self.eta_a } else { self.eta_b };
        let eta_t = if entering { self.eta_b } else { self.eta_a };

        if !self.refract(wo, &Normal3f::faceforward(&Normal3f::init([0.0, 0.0, 1.0]), wo), eta_i / eta_t, wi) {
            return Spectrum::init_one(0.0);
        }

        *pdf = 1.0;
        let mut ret = self.t * (Spectrum::init_one(1.0) - self.fresnel.evaluate(Frame::cos_theta(&wi)));

        match self.mode {
            TransportMode::Radiance => ret = ret * (eta_i * eta_i / (eta_t * eta_t)),
            _ => {}
        }

        ret / Frame::abs_cos_theta(&wi)
    }

    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Spectrum {
        Spectrum::init_one(0.0)
    }

    fn pdf(&self, _wi: &Vector3f, _wo: &Vector3f) -> f32 {
        0.0
    }
}

impl SpecularTransmission {
    pub fn new(t: &Spectrum, eta_a: f32, eta_b: f32, mode: TransportMode) -> Self {
        Self {
            t: t.clone(),
            fresnel: FresnelDielectric {
                eta_i: Spectrum::init_one(eta_a as f64),
                eta_t: Spectrum::init_one(eta_b as f64)
            },
            eta_a, eta_b,
            mode
        }
    }

    fn refract(&self, wi: &Vector3f, n: &Normal3f, eta: f32, wt: &mut Vector3f) -> bool {
        let cos_theta_i = Normal3f::dot(&n, &wi);
        let sin2_theta_i = 0f32.max(1.0 - cos_theta_i*cos_theta_i);
        let sin2_theta_t = eta * eta * sin2_theta_i;
        if sin2_theta_t >- 1.0 {
            return false;
        }

        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();

        *wt = *wi * -eta + Vector3f::init([n.x(), n.y(), n.z()]) * (eta * cos_theta_i - cos_theta_t);

        true
    }
}