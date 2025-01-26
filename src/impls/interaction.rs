use crate::common::*;

#[derive(Clone)]
pub struct SurfaceInteraction {
    pub p: Point3f,
    pub t: f32,
    pub wo: Vector3f,
    pub n: Normal3f,
    pub medium_interface: MediumInterface,

    pub uv: Point2f,
    pub dpdu: Vector3f, pub dpdv: Vector3f, pub dndu: Normal3f, pub dndv: Normal3f,
    pub shape: Option<Arc<dyn Shape>>, 
    pub shading: Shading,
    pub dpdx: Vector3f, pub dpdy: Vector3f,
    pub dudx: f32, pub dvdx: f32, pub dudy: f32, pub dvdy: f32,

    // TODO add BSDF, BSSRDF
    pub primitive: Option<Arc<dyn Primitive>>,
}

impl Interaction for SurfaceInteraction {
    fn p(&self) -> Point3f { self.p }
    fn t(&self) -> f32 { self.t }
    fn n(&self) -> Normal3f { self.n }
    fn wo(&self) -> Vector3f { self.wo }
    fn medium_interface(&self) -> &MediumInterface { &self.medium_interface }
}

impl SurfaceInteraction {
    pub fn new() -> Self {
        Self {
            p: Point3f::new(),
            t: 0f32,
            wo: Vector3f::new(),
            n: Normal3f::new(),
            medium_interface: MediumInterface::new(),

            uv: Point2f::new(),
            dpdu: Vector3f::new(), dpdv: Vector3f::new(), dndu: Normal3f::new(), dndv: Normal3f::new(),
            shape: None,
            shading: Shading::new(),
            dpdx: Vector3f::new(), dpdy: Vector3f::new(),
            dudx: 0f32, dudy: 0f32, dvdx: 0f32, dvdy: 0f32,
            primitive: None
        }
    }


    pub fn init(p: Point3f, uv: Point2f, wo: Vector3f, dpdu: Vector3f, dpdv: Vector3f, dndu: Normal3f, dndv: Normal3f, t: f32) -> Self {
        let mut ret = Self::new();
        let c_p = Vector3f::cross(&dpdu, &dpdv);
        let n = Normal3f::normalize(&Normal3f::init([c_p.x(), c_p.y(), c_p.z()]));

        ret.p = p;
        ret.t = t;
        ret.wo = wo;
        ret.n = n;
        ret.medium_interface = MediumInterface::new();
        
        ret.uv = uv;
        ret.dpdu = dpdu;
        ret.dpdv = dpdv;
        ret.dndu = dndu;
        ret.dndv = dndv;
        ret.shape = None;
        ret.shading.n = n;
        ret.shading.dpdu = dpdu;
        ret.shading.dpdv = dpdv;
        ret.shading.dndu = dndu;
        ret.shading.dndv = dndv;

        if let Some(s) = &ret.shape {
        if s.reverse_orientation() ^ s.transform_swaps_handedness() {
            ret.n = ret.n * -1.0;
            ret.shading.n = ret.shading.n * -1.0;
            }
        }

        ret
    }

    pub fn set_shape(&mut self, shape: Arc<dyn Shape>) {
        self.shape = Some(shape);
    }

    pub fn set_primitive(&mut self, primitive: Arc<dyn Primitive>) {
        self.primitive = Some(primitive);
    }

    pub fn set_shading_geometry(&mut self, dpdus: Vector3f, dpdvs: Vector3f, dndus: Normal3f, dndvs: Normal3f, orientation_is_authority: bool) {
        let c_p = Vector3f::cross(&dpdus, &dpdvs);
        let n_temp = Normal3f::normalize(&Normal3f::init([c_p.x(), c_p.y(), c_p.z()]));

        self.shading.n = n_temp;

        if let Some(s) = &self.shape {
        if s.reverse_orientation() ^ s.transform_swaps_handedness() {
            self.shading.n = -self.shading.n;
        }}


        if orientation_is_authority {
            let temp = Vector3f::init([self.shading.n.x(), self.shading.n.y(), self.shading.n.z()]);
            self.n = Normal3f::faceforward(&self.n, &temp);
        } else {
            let temp = Vector3f::init([self.n.x(), self.n.y(), self.n.z()]);
            self.shading.n = Normal3f::faceforward(&self.shading.n, &temp);
        }

        self.shading.dpdu = dpdus;
        self.shading.dpdv = dpdvs;
        self.shading.dndu = dndus;
        self.shading.dndv = dndvs;
    }

    pub fn to_string(&self) -> String {
        format!("Intersection: [\n  p: {},\n  t: {},\n  wo: {},\n  n: {},\n  uv: {}\n]", self.p.to_string(), self.t, self.wo.to_string(), self.n.to_string(), self.uv.to_string())
    }
}