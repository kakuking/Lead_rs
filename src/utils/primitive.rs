use crate::common::*;

pub trait Primitive{
    fn world_bound(&self) -> Bounds3f;
    fn intersect(&self, ray: &Ray, its: &mut SurfaceInteraction) -> bool;
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn get_area_light(&self) -> Arc<dyn AreaLight>;
    fn get_material(&self) -> Arc<dyn Material>;
    fn compute_scattering_functions(&self, its: &SurfaceInteraction, mode: TransportMode, allow_multiple_lobes: bool);
}

pub struct GeometricPrimitive {
    pub shape: Arc<dyn Shape>,
    pub material: Arc<dyn Material>,
    pub arealight: Arc<dyn AreaLight>,
    pub medium_interface: MediumInterface
}

impl Primitive for GeometricPrimitive{
    fn world_bound(&self) -> Bounds3f {
        self.shape.get_world_bounds()
    }

    // set ray.tmax after this is called (if a hit)
    fn intersect(&self, ray: &Ray, its: &mut SurfaceInteraction) -> bool {
        let mut t_hit = 0f32;
        if !self.shape.intersect(ray, &mut t_hit, its){
            return false;
        }

        its.set_shape(self.shape.clone());
        true
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.shape.intersect_p(ray)
    }

    fn get_area_light(&self) -> Arc<dyn AreaLight> {
        self.arealight.clone()
    }

    fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    // TODO - this....................
    fn compute_scattering_functions(&self, _its: &SurfaceInteraction, _mode: TransportMode, _allow_multiple_lobes: bool) {
        
    }
}

impl GeometricPrimitive {
    pub fn init(shape: Arc<dyn Shape>, material: Arc<dyn Material>, arealight: Arc<dyn AreaLight>, medium_interface: &MediumInterface) -> Self {
        Self {
            shape: shape,
            material: material,
            arealight: arealight,
            medium_interface: MediumInterface::init(medium_interface.inside.clone(), medium_interface.outside.clone())
        }
    }
}