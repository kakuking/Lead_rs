use crate::common::*;

#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    camera_to_world: Transform,
    camera_to_screen: Transform,
    raster_to_camera: Transform,
    screen_to_raster: Transform,
    raster_to_screen: Transform,

    film: Option<Arc<Film>>,
    medium: Option<Arc<Medium>>,

    lens_radius: f32,
    focal_distance: f32,

    dx_camera: Vector3f,
    dy_camera: Vector3f
}

// Constructor
fn create_orthographic_camera(prop_list: PropertyList) -> LeadObject {
    let mut camera = OrthographicCamera::new();
    camera.init(prop_list);
    LeadObject::Camera(Arc::new(camera))
}

impl Camera for OrthographicCamera {
    fn camera_to_world(&self) -> &Transform { &self.camera_to_world }
    fn film(&self) -> Option<Arc<Film>> { self.film.clone() }
    fn medium(&self) -> Option<Arc<Medium>> { self.medium.clone() }

    fn set_camera_to_world(&mut self, t: &Transform) { self.camera_to_world = t.clone(); }
    fn set_film(&mut self, film: Arc<Film>) { self.film = Some(film); }
    fn set_medium(&mut self, medium: Arc<Medium>) { self.medium = Some(medium); }

    fn generate_ray(&self, ray: &mut Ray, sample: &CameraSample) -> f32 {
        let p_film = Point3f::init([sample.p_film.x(), sample.p_film.y(), 0.0]);
        let p_camera = &self.raster_to_camera * p_film;

        *ray = Ray::init(&p_camera, &Vector3f::init([0.0, 0.0, 1.0]), EPSILON, INFINITY);
        
        if self.lens_radius > 0.0 {
            let p_lens = Warp::sample_concentric_disk(sample.p_lens) * self.lens_radius;

            let ft = self.focal_distance / ray.d.z();
            let p_focus = ray.at(ft);

            (*ray).o = Point3f::init([p_lens.x(), p_lens.y(), 0.0]);
            (*ray).d = Vector3f::normalize(&(p_focus - ray.o));
        }

        (*ray).medium = self.medium.clone();
        *ray = &self.camera_to_world * &(*ray);

        1.0
    }

    fn generate_ray_differential(&self, rd: &mut RayDifferential, sample: &CameraSample) -> f32 {
        let p_film = Point3f::init([sample.p_film.x(), sample.p_film.y(), 0.0]);
        let p_camera = &self.raster_to_camera * p_film;

        *rd = RayDifferential::init(&p_camera, &Vector3f::init([0.0, 0.0, 1.0]), EPSILON, INFINITY);

        if self.lens_radius > 0.0 {
            let p_lens = Warp::sample_concentric_disk(sample.p_lens) * self.lens_radius;

            let ft = self.focal_distance / rd.d.z();
            let mut p_focus = rd.at(ft);

            (*rd).o = Point3f::init([p_lens.x(), p_lens.y(), 0.0]);
            (*rd).d = Vector3f::normalize(&(p_focus - rd.o));

            p_focus = p_camera + self.dx_camera + Vector3f::init([0.0, 0.0, 1.0]) * ft;
            (*rd).rx_o = Point3f::init([p_lens.x(), p_lens.y(), 0.0]);
            (*rd).rx_d = Vector3f::normalize(&(p_focus - rd.rx_o));

            p_focus = p_camera + self.dy_camera + Vector3f::init([0.0, 0.0, 1.0]) * ft;
            (*rd).ry_o = Point3f::init([p_lens.x(), p_lens.y(), 0.0]);
            (*rd).ry_d = Vector3f::normalize(&(p_focus - rd.ry_o));
        } else {
            (*rd).rx_o = rd.o + self.dx_camera;
            (*rd).ry_o = rd.o + self.dy_camera;
            (*rd).rx_d = rd.d; 
            (*rd).ry_d = rd.d;
        }

        (*rd).medium = self.medium.clone();
        (*rd).has_differential = true;

        *rd = &self.camera_to_world * &(*rd);

        1.0
    }
    
    // TODO - IMPLEMENT ALLAT
    fn pdf_we(&self, _ray: &Ray, _pdf_pos: &mut f32, _pdf_dir: &mut f32) {
        
    }

    fn sample_wi(&self, _reference: Arc<dyn Interaction>, _u: &Point2f, _wi: &mut Vector3f,_pdf: &mut f32, _p_raster: &mut Point2f, _vis: &mut VisibilityTester) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0], SpectrumType::Reflectance)
    }

    fn we(&self, _ray: &Ray, _p_raster_2: &mut Point2f) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0], SpectrumType::Reflectance)
    }
}   

impl ProjectiveCamera for OrthographicCamera {
    fn camera_to_screen(&self) -> &Transform { &self.camera_to_screen }
    fn raster_to_camera(&self) -> &Transform { &self.raster_to_camera }
    fn screen_to_raster(&self) -> &Transform { &self.screen_to_raster }
    fn raster_to_screen(&self) -> &Transform { &self.raster_to_screen }
    fn set_camera_to_screen(&mut self, t: &Transform) { self.camera_to_screen = t.clone(); }
    fn set_raster_to_camera(&mut self, t: &Transform) { self.raster_to_camera = t.clone(); }
    fn set_screen_to_raster(&mut self, t: &Transform) { self.screen_to_raster = t.clone(); }
    fn set_raster_to_screen(&mut self, t: &Transform) { self.raster_to_screen = t.clone(); }

    fn lens_radius(&self) -> f32 { self.lens_radius }
    fn focal_distance(&self) -> f32 { self.focal_distance }
    fn set_lens_radius(&mut self, lr: f32) { self.lens_radius = lr; }
    fn set_focal_distance(&mut self, fd: f32) { self.focal_distance = fd; }
}

impl LeadObjectTrait for OrthographicCamera {
    // TODO ACTUALLY GET SCREEN WINDOW, Film, and MEDIUM here
    fn init(&mut self, prop_list: PropertyList) {
        let lookat = prop_list.get_point3("lookat", Point3f::new());
        let origin = prop_list.get_point3("eye", Point3f::init([0.0, 0.0, -1.0]));
        let up = prop_list.get_vector3("up", Vector3f::init([0.0, 1.0, 0.0]));

        let camera_to_world = Transform::look_at(&origin, &lookat, &up);
        let lens_r = prop_list.get_float("lens_radius", 0.0);   // 0.0 means no depth of field
        let focal_d = prop_list.get_float("focal_distance", 1.0);

        let mut screen_window = Bounds2f::new();
        screen_window.p_min = Point2f::init([0.0, 0.0]);
        screen_window.p_max = Point2f::init([800.0, 600.0]);

        let film = Film{
            full_resolution: Point2f::init([800.0, 600.0])
        };
        let medium = Medium {};

        self.init_projective_camera(camera_to_world, Self::ortho_projection_matrix(0.0, 1.0), screen_window, lens_r, focal_d, Arc::new(film), Arc::new(medium));

        self.dx_camera = &self.raster_to_camera * Vector3f::init([1.0, 0.0, 0.0]);
        self.dy_camera = &self.raster_to_camera * Vector3f::init([0.0, 1.0, 0.0]);
    }

    fn activate(&mut self) {
        
    }

    fn add_child(&mut self, _child: &mut LeadObject) {
        panic!("Cannot add child to orthographic camera!");
    }

    fn to_string(&self) -> String {
        format!("Orthographic Camera[]")
    }
}

impl OrthographicCamera {
    pub fn new() -> Self {
        Self {
            camera_to_world:  Transform::new(),
            camera_to_screen: Transform::new(),
            raster_to_camera: Transform::new(),
            screen_to_raster: Transform::new(),
            raster_to_screen: Transform::new(),

            film: None,
            medium: None,

            lens_radius: 0.0, focal_distance: 1.0,
            dx_camera: Vector3f::new(), dy_camera: Vector3f::new()
        }
    }

    fn ortho_projection_matrix(z_near: f32, z_far: f32) -> Transform {
        Transform::scale(&Vector3f::init([1.0, 1.0, 1.0 / (z_far - z_near)]))
        * Transform::translate(&Vector3f::init([0.0, 0.0, -z_near]))
    }
}

register_struct!("orthographic", create_orthographic_camera);