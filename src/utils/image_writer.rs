use crate::common::*;
use exr::prelude::*;

pub fn write_image_to_file(rgb: Vec<f32>, filename: String, resolution: &Point2f) {
    let width = resolution.x() as usize;
    let height = resolution.y() as usize;
    write_rgba_file(
        filename + ".exr", 
        width, 
        height, 
        |x, y| {
            let idx = (y * width + x) * 3;
            let r = rgb[idx];
            let g = rgb[idx + 1];
            let b = rgb[idx + 2];
            let a = 1.0;

            (r, g, b, a)
        }
    ).unwrap();
}