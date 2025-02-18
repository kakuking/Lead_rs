use crate::common::*;

pub struct SpecularReflection {
    r: Spectrum,
    fresnel: Fresnel
}

impl BxDF for SpecularReflection {
    fn bxdf_type(&self) -> u32 {
        BxDFType::BSDFReflection.to_u32() | BxDFType::BSDFSpecular.to_u32()
    }

    fn sample_f(&self, wo: &Vector3f, wi: &mut Vector3f, _sample: &Point2f, pdf: &mut f32, _sampled_type: Option<BxDFType>) -> Spectrum {
        // we are in frame of the normal, so reflection is very very easy
        *wi = Vector3f::init([-wo.x(), -wo.y(), wo.z()]);
        *pdf = 1.0;

        self.fresnel.evaluate(Frame::cos_theta(&wi)) * self.r / Frame::abs_cos_theta(&wi) as f64
    }

    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Spectrum {
        Spectrum::init_one(0.0)
    }

    fn pdf(&self, _wi: &Vector3f, _wo: &Vector3f) -> f32 {
        0.0
    }
}

impl SpecularReflection {
    pub fn new(r: &Spectrum, fr: Fresnel) -> Self {
        Self {
            r: r.clone(),
            fresnel: fr
        }
    }
}