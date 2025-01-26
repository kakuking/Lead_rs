use crate::common::*;

pub trait Primitive{
    fn world_bound(&self) -> Bounds3f;
    fn intersect(&self, ray: &Ray, its: &mut SurfaceInteraction) -> bool;
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>>;
    fn get_material(&self) -> Option<Arc<dyn Material>>;
    fn compute_scattering_functions(&self, its: &SurfaceInteraction, mode: TransportMode, allow_multiple_lobes: bool);
    fn shape(&self) -> Option<Arc<dyn Shape>>;
}

pub struct GeometricPrimitive {
    pub shape: Arc<dyn Shape>,
    pub material: Option<Arc<dyn Material>>,
    pub arealight: Option<Arc<dyn AreaLight>>,
    pub medium_interface: Option<MediumInterface>
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

    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>> {
        match &self.arealight {
            Some(al) => { return Some(al.clone()); }
            None => {return None;}
        }
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        match &self.material {
            Some(mat) => { return Some(mat.clone()); }
            None => { return None; }
        }
    }

    // TODO - this....................
    fn compute_scattering_functions(&self, _its: &SurfaceInteraction, _mode: TransportMode, _allow_multiple_lobes: bool) {
        
    }

    fn shape(&self) -> Option<Arc<dyn Shape>> {
        Some(self.shape.clone())
    }
}

impl GeometricPrimitive {
    pub fn init(shape: Arc<dyn Shape>, material: Arc<dyn Material>, arealight: Arc<dyn AreaLight>, medium_interface: &MediumInterface) -> Self {
        Self {
            shape: shape,
            material: Some(material),
            arealight: Some(arealight),
            medium_interface: Some(MediumInterface::init(medium_interface.inside.clone(), medium_interface.outside.clone()))
        }
    }

    pub fn init_shape(shape: Arc<dyn Shape>) -> Self {
        Self {
            shape: shape,
            material: None,
            arealight: None,
            medium_interface: None
        }
    }
}