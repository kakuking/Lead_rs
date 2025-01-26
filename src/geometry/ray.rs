use crate::common::*;

pub struct Ray {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_min: f32,
    pub t_max: f32,
    pub medium: Option<Arc<Medium>>
}

pub struct RayDifferential {
    pub o: Point3f,
    pub d: Vector3f,
    pub t_min: f32,
    pub t_max: f32,
    pub medium: Option<Arc<Medium>>,

    pub rx_o: Point3f,
    pub ry_o: Point3f,
    pub rx_d: Vector3f,
    pub ry_d: Vector3f,
    pub has_differential: bool
}

impl Ray {
    pub fn new() -> Self {
        Self{
            o: Point3f::new(),
            d: Vector3f::new(),
            t_min: EPSILON,
            t_max: INFINITY,
            medium: None
        }
    }

    pub fn init(origin: &Point3f, dir: &Vector3f, t_min: f32, t_max: f32) -> Self {
        Self {
            o: origin.clone(),
            d: Vector3f::normalize(dir),
            t_min: t_min,
            t_max: t_max,
            medium: None
        }
    }

    pub fn at(&self, t: f32) -> Point3f {
        self.o + self.d * t
    }

    pub fn has_nan(&self) -> bool {
        self.o.has_nan() || self.d.has_nan() || self.t_min.is_nan() || self.t_max.is_nan()
    }

    pub fn to_string(&self) -> String {
        format!("Ray[o: {}, d: {}, t_min: {}, t_min: {}]", self.o.to_string(), self.d.to_string(), self.t_min, self.t_max)
    }
}

impl RayDifferential {
    pub fn new() -> Self {
        Self{
            o: Point3f::new(),
            d: Vector3f::new(),
            t_min: EPSILON,
            t_max: INFINITY,
            medium: None,

            rx_o: Point3f::new(),
            ry_o: Point3f::new(),
            rx_d: Vector3f::new(),
            ry_d: Vector3f::new(),
            has_differential: true
        }
    }

    pub fn init(origin: &Point3f, dir: &Vector3f, t_min: f32, t_max: f32) -> Self {
        Self {
            o: origin.clone(),
            d: dir.clone(),
            t_min: t_min,
            t_max: t_max,
            medium: None,
            
            rx_o: Point3f::new(),
            ry_o: Point3f::new(),
            rx_d: Vector3f::new(),
            ry_d: Vector3f::new(),
            has_differential: true
        }
    }

    pub fn at(&self, t: f32) -> Point3f {
        self.o + self.d * t
    }

    pub fn has_nan(&self) -> bool {
        let ray_part = self.o.has_nan() || self.d.has_nan() || 
        self.t_min.is_nan() || self.t_max.is_nan();

        let diff_part = self.has_differential && (self.rx_d.has_nan() || self.rx_o.has_nan() ||
        self.ry_d.has_nan() || self.ry_o.has_nan());

        ray_part || diff_part
    }

    pub fn to_string(&self) -> String {
        format!("Ray[o: {}, d: {}, t_min: {}, t_max: {}]", self.o.to_string(), self.d.to_string(), self.t_min, self.t_max)
    }

    pub fn scale_differentials(&mut self, s: f32) {
        self.rx_o = self.o + (self.rx_o - self.o) * s;
        self.ry_o = self.o + (self.ry_o - self.o) * s;
        self.rx_d = self.d + (self.rx_d - self.d) * s;
        self.ry_d = self.d + (self.ry_d - self.d) * s;
    }
}