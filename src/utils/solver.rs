pub struct Solver { }

impl Solver {
    pub fn quadratic(a: f32, b: f32, c: f32, r_1: &mut Option<f32>, r_2: &mut Option<f32>) -> bool {
        
        let det = b*b - 4.0*a*c;
        if det < 0.0{
            return false;
        }

        let sub = det.sqrt();
        let r1 = (-b - sub) / (2.0 * a);
        let r2 = (-b + sub) / (2.0 * a);

        *r_1 = Some(r1);
        *r_2 = Some(r2);

        true
    }
}