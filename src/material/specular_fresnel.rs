use crate::common::*;

pub struct FresnelSpecular {
    pub r: Spectrum, 
    pub t: Spectrum,
    pub fresnel: FresnelDielectric,
    pub eta_a: f32, 
    pub eta_b: f32,
    pub mode: TransportMode
}

impl BxDF for FresnelSpecular {
    fn bxdf_type(&self) -> u32 {
        BxDFType::BSDFTtransmission.to_u32() | BxDFType::BSDFReflection.to_u32() | BxDFType::BSDFSpecular.to_u32()
    }

    fn sample_f(&self, _wo: &Vector3f, _wi: &mut Vector3f, _sample: &Point2f, _pdf: &mut f32, _sampled_type: Option<BxDFType>) -> Spectrum {
        // TODO this in chapter 14
        Spectrum::init_one(0.0)
    }

    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Spectrum {
        Spectrum::init_one(0.0)
    }

    fn pdf(&self, _wi: &Vector3f, _wo: &Vector3f) -> f32 {
        0.0
    }
}

impl FresnelSpecular {
    pub fn new(r: &Spectrum, t: &Spectrum, eta_a: f32, eta_b: f32, mode: TransportMode) -> Self {
        Self {
            r: r.clone(),
            t: t.clone(),
            fresnel: FresnelDielectric {
                eta_i: Spectrum::init_one(eta_a as f64),
                eta_t: Spectrum::init_one(eta_b as f64)
            },
            eta_a, eta_b,
            mode
        }
    }
}