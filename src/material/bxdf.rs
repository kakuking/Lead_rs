use crate::common::*;

pub enum BxDFType {
    BSDFReflection = 1 << 0,
    BSDFTtransmission = 1 << 1,
    BSDFDiffuse = 1 << 2,
    BSDFGlossy = 1 << 3,
    BSDFSpecular = 1 << 4,
    BSDFAll = 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl BxDFType {
    pub fn to_u32(&self) -> u32 {
        match self {
            BxDFType::BSDFReflection => 1 << 0,
            BxDFType::BSDFTtransmission => 1 << 1,
            BxDFType::BSDFDiffuse => 1 << 2,
            BxDFType::BSDFGlossy => 1 << 3,
            BxDFType::BSDFSpecular => 1 << 4,
            BxDFType::BSDFAll => 1 << 0 | 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
        }
    }
}

pub trait BxDF {
    fn bxdf_type(&self) -> u32;

    // fn new(&self, bxdf_type: BxDFType) -> Self;
    fn f(&self, wo: &Vector3f, wi: &Vector3f) -> Spectrum;
    fn sample_f(&self, wo: &Vector3f, wi: &mut Vector3f, sample: &Point2f, pdf: &mut f32, sampled_type: Option<BxDFType>) -> Spectrum;
    fn rho(&self, _wo: &Vector3f, _samples: &Vec<Point2f>) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0])
    }
    fn rho_multi_sample(&self, _samples_1: &Vec<Point2f>, _samples_2: &Vec<Point2f>) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0])
    }
    fn pdf(&self, wi: &Vector3f, wo: &Vector3f) -> f32;
    fn matches_flags(&self, t: BxDFType) -> bool {
        t.to_u32() & self.bxdf_type() == self.bxdf_type()
    }
}