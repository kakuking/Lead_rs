use crate::common::*;

#[derive(Debug, Clone)]
pub struct EnvironmentCamera {
    camera_to_world: Transform,
    film: Option<Arc<Film>>,
    medium: Option<Arc<Medium>>,
}

// Constructor
fn create_environment_camera(prop_list: PropertyList) -> LeadObject {
    let mut camera = EnvironmentCamera::new();
    camera.init(prop_list);
    LeadObject::Camera(Arc::new(camera))
}

impl Camera for EnvironmentCamera {
    fn camera_to_world(&self) -> &Transform { &self.camera_to_world }
    fn film(&self) -> Option<Arc<Film>> { self.film.clone() }
    fn medium(&self) -> Option<Arc<Medium>> { self.medium.clone() }

    fn set_camera_to_world(&mut self, t: &Transform) { self.camera_to_world = t.clone(); }
    fn set_film(&mut self, film: Arc<Film>) { self.film = Some(film); }
    fn set_medium(&mut self, medium: Arc<Medium>) { self.medium = Some(medium); }

    fn generate_ray(&self, ray: &mut Ray, sample: &CameraSample) -> f32 {
        let theta = M_PI * sample.p_film.y() / self.film().unwrap().full_resolution.y();
        let phi = 2.0 * M_PI * sample.p_film.x() / self.film().unwrap().full_resolution.x();
        let dir = Vector3f::init([theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin()]);

        (*ray).o = Point3f::new();
        (*ray).d = dir;
        (*ray).t_min = EPSILON;
        (*ray).t_max = INFINITY;
        (*ray).medium = self.medium.clone();

        1.0
    }

    // TODO - IMPLEMENT ALLAT
    fn pdf_we(&self, _ray: &Ray, _pdf_pos: &mut f32, _pdf_dir: &mut f32) {
        
    }

    fn sample_wi(&self, _reference: Arc<dyn Interaction>, _u: &Point2f, _wi: &mut Vector3f,_pdf: &mut f32, _p_raster: &mut Point2f, _vis: &mut VisibilityTester) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0])
    }

    fn we(&self, _ray: &Ray, _p_raster_2: &mut Point2f) -> Spectrum {
        Spectrum::from_rgb([0.0, 0.0, 0.0])
    }
}   

impl LeadObjectTrait for EnvironmentCamera {
    // TODO ACTUALLY GET SCREEN WINDOW, Film, and MEDIUM here
    fn init(&mut self, prop_list: PropertyList) {
        let lookat = prop_list.get_point3("lookat", Point3f::new());
        let origin = prop_list.get_point3("eye", Point3f::init([0.0, 0.0, -1.0]));
        let up = prop_list.get_vector3("up", Vector3f::init([0.0, 1.0, 0.0]));

        let camera_to_world = Transform::look_at(&origin, &lookat, &up);
        let film = Film::new();
        let medium = Medium {};

        self.set_medium(Arc::new(medium));
        self.set_film(Arc::new(film));
        self.set_camera_to_world(&camera_to_world);
    }

    fn activate(&mut self) {
        
    }

    fn add_child(&mut self, _child: &mut LeadObject) {
        panic!("Cannot add child to perspective camera!");
    }

    fn to_string(&self) -> String {
        format!("Environment Camera[]")
    }
}

impl EnvironmentCamera {
    pub fn new() -> Self {
        Self {
            camera_to_world:  Transform::new(),
            film: None,
            medium: None,
        }
    }
}

register_struct!("environment", create_environment_camera);