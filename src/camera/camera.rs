use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub struct CameraSample{
    pub p_film: Point2f,
    pub p_lens: Point2f,
}

pub trait Camera: LeadObjectTrait {
    fn camera_to_world(&self) -> &Transform;
    fn film(&self) -> Option<Arc<Film>>;
    fn medium(&self) -> Option<Arc<Medium>>;

    fn set_camera_to_world(&mut self, t: &Transform);
    fn set_film(&mut self, film: Arc<Film>);
    fn set_medium(&mut self, medium: Arc<Medium>);

    fn generate_ray(&self, ray: &mut Ray, sample: &CameraSample) -> f32;
    fn generate_ray_differential(&self, rd: &mut RayDifferential, sample: &CameraSample) -> f32 {
        let mut r: Ray = Ray::new();
        let wt = self.generate_ray(&mut r, sample);
        rd.o = r.o;
        rd.d = r.d;
        rd.t_min = r.t_min; rd.t_max = r.t_max;

        let mut s_shift = sample.clone();
        s_shift.p_film[0] += 1.0;
        let mut rx: Ray = Ray::new();
        let wtx = self.generate_ray(&mut rx, &s_shift);
        if wtx == 0.0 {
            return 0.0;
        }
        rd.rx_o = rx.o;
        rd.rx_d = rx.d;

        s_shift.p_film[0] -= 1.0;
        s_shift.p_film[1] += 1.0;
        let mut ry: Ray = Ray::new();
        let wty= self.generate_ray(&mut ry, &s_shift);
        if wty == 0.0 {
            return 0.0;
        }
        rd.ry_o = ry.o;
        rd.ry_d = ry.d;

        rd.has_differential = true;

        wt
    }
    fn we(&self, ray: &Ray, p_raster_2: &mut Point2f) -> Spectrum;
    fn pdf_we(&self, ray: &Ray, pdf_pos: &mut f32, pdf_dir: &mut f32);
    fn sample_wi(&self, reference: Arc<dyn Interaction>, u: &Point2f, wi: &mut Vector3f, pdf: &mut f32, p_raster: &mut Point2f, vis: &mut VisibilityTester) -> Spectrum;
}

pub trait ProjectiveCamera: Camera {
    fn camera_to_screen(&self) -> &Transform;
    fn raster_to_camera(&self) -> &Transform;
    fn screen_to_raster(&self) -> &Transform;
    fn raster_to_screen(&self) -> &Transform;
    fn lens_radius(&self) -> f32;
    fn focal_distance(&self) -> f32;

    fn set_camera_to_screen(&mut self, t: &Transform);
    fn set_raster_to_camera(&mut self, t: &Transform);
    fn set_screen_to_raster(&mut self, t: &Transform);
    fn set_raster_to_screen(&mut self, t: &Transform);
    fn set_lens_radius(&mut self, lr: f32);
    fn set_focal_distance(&mut self, fd: f32);

    fn init_projective_camera(&mut self, camera_to_world: Transform, camera_to_screen: Transform, screen_window: Bounds2f, lens_r: f32, focal_d: f32, film: Arc<Film>, medium: Arc<Medium>) {
        self.set_camera_to_world(&camera_to_world);
        self.set_film(film.clone());
        self.set_medium(medium);
        self.set_camera_to_screen(&camera_to_screen);

        self.set_lens_radius(lens_r);
        self.set_focal_distance(focal_d);

        let mut screen_to_raster = Transform::scale(&Vector3f::init([film.full_resolution.x(), film.full_resolution.y(), 1.0]));
        
        let screen_window_vec = Vector3f::init([1.0 / (screen_window.p_max.x() - screen_window.p_min.x()), 1.0 / (screen_window.p_max.y() - screen_window.p_min.y()), 1.0]);
        screen_to_raster = screen_to_raster * Transform::scale(&screen_window_vec);
        screen_to_raster = screen_to_raster * Transform::translate(&Vector3f::init([-screen_window.p_min.x(), -screen_window.p_max.y(), 0.0]));
        self.set_screen_to_raster(&screen_to_raster);
        self.set_raster_to_screen(&screen_to_raster.inverse());
        self.set_raster_to_camera(&(self.camera_to_screen().inverse() * screen_to_raster.inverse()));
    }
}