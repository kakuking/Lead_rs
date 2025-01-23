use crate::common::*;

pub struct Warp;

impl Warp {
    pub fn sample_concentric_disk(u: Point2f) -> Point2f {
        let u_offset = u * 2.0 - Vector2f::init([1.0, 1.0]);

        if u_offset.x() == 0.0 && u_offset.y() == 0.0 {
            return Point2f::new();
        }

        let (theta, r) = {
            if u_offset.x().abs() > u_offset.y().abs() {
                (M_PI * 0.25 * (u_offset.y() / u_offset.x()), u_offset.x())
            } else {
                (M_PI * (0.5 - 0.25 * (u_offset.x() / u_offset.y())), u_offset.y())
            }
        };


        Point2f::init([theta.cos(), theta.sin()]) * r
    }

    pub fn sample_uniform_disk(u: Point2f) -> Point2f {
        let r = u.x().sqrt();
        let theta = 2.0 * M_PI * u.y();

        Point2f::init([theta.cos(), theta.sin()]) * r
    }
}

