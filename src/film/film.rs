use crate::common::*;
use atomic_float::AtomicF64;
use box_filter::BoxFilter;

#[derive(Debug)]
struct Pixel {
    xyz: [f64; 3],
    filter_weight_sum: f32,
    splat_xyz: [AtomicF64; 3],
}

impl Pixel{
    pub fn new() -> Self {
        Self {
            xyz: [0f64; 3],
            filter_weight_sum: 0f32,
            splat_xyz: [0f64.into(), 0f64.into(), 0f64.into()]
        }
    }
}

#[derive(Debug)]
pub struct Film {
    pub full_resolution: Point2f,
    pub diagonal: f32,
    pub filter: Arc<dyn Filter>,
    pub filename: String,
    pub cropped_pixel_bounds: Bounds2f,

    pixels: Mutex<Vec<Pixel>>,
    filter_table_width: usize,  // Always 16
    filter_table: [f32; 256usize],   // len is 16^2
    scale: f32,
    // mutex: Mutex<u8>,
    // dont need mutex, can wrap the whole hting ina arc<mutex<film>>
}

impl Film{
    pub fn new() -> Self {
        Self {
            full_resolution: Point2f::new(),
            diagonal: 0f32,
            filter: Arc::new(BoxFilter::new(&Vector2f::new())),
            filename: "".to_string(),
            scale: 0f32,

            cropped_pixel_bounds: Bounds2f::new(),
            pixels: Mutex::new(vec![Pixel::new()]),
            filter_table: [0f32; 256],
            filter_table_width: 16usize,
            // mutex: Mutex::from(1u8)
        }
    }

    pub fn init(resolution: &Point2f, crop_window: &Bounds2f, filter: Arc<dyn Filter>, diagonal: f32, filename: String, scale: f32) -> Self {
        let cropped_pixel_bounds = Bounds2f::init(
            &Point2f::init([(resolution.x() * crop_window.p_min.x()).ceil(), (resolution.y() * crop_window.p_min.y()).ceil()]), 
            &Point2f::init([(resolution.x() * crop_window.p_max.x()).ceil(), (resolution.y() * crop_window.p_max.y()).ceil()]));

        let mut pixels_inner: Vec<Pixel> = Vec::new();
        for _ in 0..cropped_pixel_bounds.area() as usize {
            pixels_inner.push(Pixel::new());
        }
        let pixels = Mutex::from(pixels_inner);

        let mut filter_table: Vec<f32> = Vec::new();
        let filter_table_width = 16usize;
        for y in 0..filter_table_width {
            for x in 0..filter_table_width {
                let mut p = Point2f::new();
                p[0] = (x as f32 + 0.5) * filter.radius().x() / filter_table_width as f32;
                p[1] = (y as f32 + 0.5) * filter.radius().y() / filter_table_width as f32;
                filter_table.push(filter.evaluate(&p));
            } 
        }

        Self {
            full_resolution: *resolution,
            diagonal: diagonal * 0.001,
            filter: filter.clone(),
            filename: filename,
            scale: scale,

            cropped_pixel_bounds: cropped_pixel_bounds,
            pixels: pixels,
            filter_table: filter_table.try_into().unwrap(), // always is 256 so we good
            filter_table_width: filter_table_width,
            // mutex: Mutex::from(1u8)
        }
    }

    pub fn get_sample_bounds(&self) -> Bounds2f {
        let p_min = Point2f::floor(
            &(Point2f::init_copy(&self.cropped_pixel_bounds.p_min)
            + Vector2f::init([0.5, 0.5]) - self.filter.radius())
        );
        let p_max = Point2f::ceil(
            &(Point2f::init_copy(&self.cropped_pixel_bounds.p_max)
            - Vector2f::init([0.5, 0.5]) + self.filter.radius())
        );

        Bounds2f::init(&p_min, &p_max)
    }

    pub fn get_physical_extent(&self) -> Bounds2f {
        let aspect = self.full_resolution.y() / self.full_resolution.x();
        let x = (self.diagonal*self.diagonal / (1.0 + aspect*aspect)).sqrt();
        let y = aspect * x;

        let p_min = Point2f::init([-x/2.0, -y/2.0]);
        let p_max = Point2f::init([x/2.0, y/2.0]);

        Bounds2f::init(&p_min, &p_max)
    }

    pub fn get_film_tile(&self, sample_bounds: &Bounds2f) -> Arc<FilmTile> {
        let half_pixel = Vector2f::init([0.5, 0.5]);
        let p0 = (sample_bounds.p_min - half_pixel - self.filter.radius()).ceil();
        let p1 = (sample_bounds.p_max - half_pixel + self.filter.radius()).ceil() + Point2f::init([1.0, 1.0]);

        let tile_pixel_bounds = Bounds2f::intersect(&Bounds2f::init(&p0, &p1), &self.cropped_pixel_bounds);

        let ret = FilmTile::new( &tile_pixel_bounds, &self.filter.radius(), self.filter_table.to_vec(), self.filter_table_width as f32);

        Arc::new(ret)
    }

    pub fn merge_film_title(&mut self, tile: Arc<FilmTile>) {
        let mut pixels = self.pixels.lock().unwrap();

        for pixel in tile.get_pixel_bounds().iter() {
            let tile_pixel = tile.get_pixel_const(&pixel);
            let merge_pixel_offset = self.get_pixel_offset(&pixel);

            let mut xyz = [0f64; 3usize];
            tile_pixel.contrib_sum.to_xyz(&mut xyz);

            let merge_pixel = &mut pixels[merge_pixel_offset];
            for i in 0..3 {
                merge_pixel.xyz[i] += xyz[i];
            }
            merge_pixel.filter_weight_sum += tile_pixel.filter_weight_sum;
        }
    }

    pub fn set_image(&self, img: &[Spectrum]) {
        let mut pixels = self.pixels.lock().unwrap();
        let n_pixels = self.cropped_pixel_bounds.area().floor() as usize;
        for i in 0..n_pixels {
            let pixel = &mut pixels[i];
            img[i].to_xyz(&mut pixel.xyz);
            pixel.filter_weight_sum = 1.0;
            pixel.splat_xyz[0] = AtomicF64::new(0f64);
            pixel.splat_xyz[1] = AtomicF64::new(0f64);
            pixel.splat_xyz[2] = AtomicF64::new(0f64);
        }
    }

    pub fn add_splat(&mut self, p: &Point2f, v: &Spectrum) {
        if !Bounds2f::inside_exclusive(p, &self.cropped_pixel_bounds) {
            return;
        }
        let mut pixels = self.pixels.lock().unwrap();
        let mut xyz = [0f64; 3usize];
        v.to_xyz(&mut xyz);
        let pixel_offset = self.get_pixel_offset(p);
        let pixel = &mut pixels[pixel_offset];

        for i in 0..3 {
            let cur_val= pixel.splat_xyz[i].load(std::sync::atomic::Ordering::Acquire);
            pixel.splat_xyz[i].store(cur_val + xyz[i], std::sync::atomic::Ordering::Release);
        }
    }

    pub fn write_image(&mut self, splat_scale: f64) {
        let mut rgb: Vec<f32> = Vec::new();
        let pixels= self.pixels.lock().unwrap();

        for p in self.cropped_pixel_bounds.iter() {
            let pixel_offset = self.get_pixel_offset(&p);
            let pixel = &pixels[pixel_offset];

            let mut pixel_rgb = [0f64; 3];
            xyz_to_rgb(pixel.xyz, &mut pixel_rgb);
            let mut x = pixel_rgb[0]; 
            let mut y = pixel_rgb[1]; 
            let mut z = pixel_rgb[2]; 

            let filter_weight_sum = pixel.filter_weight_sum;
            if filter_weight_sum != 0.0 {
                let inv_weight = (1.0 / filter_weight_sum) as f64;
                x = 0f64.max(x * inv_weight);
                y = 0f64.max(y * inv_weight);
                z = 0f64.max(z * inv_weight);
            }

            let mut splat_rgb = [0f64; 3];
            let splat_xyz = [pixel.splat_xyz[0].load(std::sync::atomic::Ordering::Acquire), pixel.splat_xyz[1].load(std::sync::atomic::Ordering::Acquire), pixel.splat_xyz[2].load(std::sync::atomic::Ordering::Acquire)];

            xyz_to_rgb(splat_xyz, &mut splat_rgb);
            x += splat_scale * splat_rgb[0];
            y += splat_scale * splat_rgb[1];
            z += splat_scale * splat_rgb[2];

            x *= self.scale as f64;
            y *= self.scale as f64;
            z *= self.scale as f64;

            rgb.push(x as f32);
            rgb.push(y as f32);
            rgb.push(z as f32);
        }

        write_image_to_file(rgb, self.filename.clone(), &self.full_resolution);
    }

    pub fn clear(&mut self) {
        let pixels = &mut self.pixels.lock().unwrap();
        for p in self.cropped_pixel_bounds.iter() {
            let pixel_offset = self.get_pixel_offset(&p);
            let pixel = &mut pixels[pixel_offset];
            for i in 0..3 {
                pixel.splat_xyz[i] = AtomicF64::new(0f64);
                pixel.xyz[i] = 0.0;
            }
            pixel.filter_weight_sum = 0.0;
        }
    }


    fn get_pixel_offset(&self, p: &Point2f) -> usize {
        let width = self.cropped_pixel_bounds.p_max.x() - self.cropped_pixel_bounds.p_min.x();
        let offset = (p.x() - self.cropped_pixel_bounds.p_min.x()) + (p.y() - self.cropped_pixel_bounds.p_min.y()) * width;

        let offset = offset as usize;
        
        offset
    }
}