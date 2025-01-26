use crate::common::*;


pub trait Shape: LeadObjectTrait{
    // First the datat types we need
    fn object_to_world(&self) -> Transform;
    fn world_to_object(&self) -> Transform;
    fn reverse_orientation(&self) -> bool;
    fn transform_swaps_handedness(&self) -> bool;

    fn get_object_bounds(&self) -> Bounds3f;
    fn get_world_bounds(&self) -> Bounds3f;
    //After calling intersect make sure to set which shape it is in the its
    fn intersect(&self, ray: &Ray, t_hit: &mut f32, its:  &mut SurfaceInteraction) -> bool;
    fn intersect_p(&self, ray: &Ray) -> bool {
        let mut t_hit: f32 = INFINITY;
        let mut its: SurfaceInteraction = SurfaceInteraction::new();

        self.intersect(ray, &mut t_hit, &mut its)
    }
    fn area(&self) -> f32;
    fn sample_u(&self, u: &Point2f) -> Box<dyn Interaction>;
    fn pdf(&self, _its: &Box<dyn Interaction>) -> f32 {
        1.0 / self.area()
    }
    fn sample(&self, _reference: &Box<dyn Interaction>, u: &Point2f) -> Box<dyn Interaction>;
    fn pdf_wi(&self, reference: &Box<dyn Interaction>, wi: &Vector3f) -> f32;
}