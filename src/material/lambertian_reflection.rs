use crate::common::*;

pub struct LambertianReflection {
    r: Spectrum,
}

impl BxDF for LambertianReflection {
    fn bxdf_type(&self) -> u32 {
        BxDFType::BSDFReflection.to_u32() | BxDFType::BSDFDiffuse.to_u32()
    }

    fn f(&self, _wo: &Vector3f, _wi: &Vector3f) -> Spectrum {
        self.r * M_INV_PI
    }

    fn rho(&self, _wo: &Vector3f, _samples: &Vec<Point2f>) -> Spectrum {
        self.r.clone()
    }

    fn rho_multi_sample(&self, _samples_1: &Vec<Point2f>, _samples_2: &Vec<Point2f>) -> Spectrum {
        self.r.clone()
    }
}

impl LambertianReflection {
    pub fn new(r: &Spectrum) -> Self {
        Self {
            r: r.clone(),
        }
    }
}