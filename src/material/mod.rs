pub mod material;
pub mod bxdf;
pub mod fresnel;
pub mod specular_reflection;

pub use material::Material;
pub use bxdf::{BxDF, BxDFType};
pub use fresnel::{Fresnel, FresnelConductor, FresnelDielectric, FresnelNoOp, fr_conductor, fr_dielectric};