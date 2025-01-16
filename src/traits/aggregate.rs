use crate::common::*;

pub trait Aggregate {
    fn get_area_light(&self) -> Arc<dyn AreaLight>;
    fn get_material(&self) -> Arc<dyn Material>;
    fn compute_scattering_functions(&self, its: &mut SurfaceInteraction, mode: &TransportMode, allow_multiple_lobes: bool);
}