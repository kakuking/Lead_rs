use crate::common::*;

#[derive(Debug, Clone)]
pub struct FilmTilePixel {
    pub contrib_sum: Spectrum,
    pub filter_weight_sum: f32
}

impl FilmTilePixel {
    pub fn new() -> Self {
        Self {
            contrib_sum: Spectrum::from_rgb([0.0, 0.0, 0.0], SpectrumType::Reflectance),
            filter_weight_sum: 0.0
        }
    }
}

#[derive(Debug)]
pub struct FilmTile{
    pub pixel_bounds: Bounds2f,
    pub filter_radius: Vector2f,
    pub inv_filter_radius: Vector2f,
    pub filter_table: Vec<f32>,
    pub pixels: Vec<FilmTilePixel>,
    pub max_sample_luminance: f32
}

impl FilmTile {
    pub fn new(pixel_bounds: &Bounds2f, filter_radius: &Vector2f, filter_table: Vec<f32>, max_sample_lum: f32) -> Self {
        let inv_rad = Vector2f::init([1.0/filter_radius.x(), 1.0/filter_radius.y()]);
        let num_pixels = (pixel_bounds.area() as usize).max(0usize);

        let mut pixels: Vec<FilmTilePixel> = Vec::new();
        for _ in 0..num_pixels {
            pixels.push(FilmTilePixel::new());
        }

        Self {
            pixel_bounds: pixel_bounds.clone(),
            filter_radius: filter_radius.clone(),
            inv_filter_radius: inv_rad,
            filter_table: filter_table,
            max_sample_luminance: max_sample_lum,
            pixels: pixels
        }
    }

    pub fn add_sample(&mut self, p_film: &Point2f, l: Spectrum, sample_weight: f32) {
        let mut l = l;
        if l.y() > self.max_sample_luminance as f64 {
            l = l * (self.max_sample_luminance as f64) * (1.0 / l.y());
        }

        let p_film_discrete = *p_film - Vector2f::init([0.5, 0.5]);
        let mut p0 = (p_film_discrete - self.filter_radius).ceil();
        let mut p1 = (p_film_discrete + self.filter_radius).ceil() + Point2f::init([1.0, 1.0]);

        p0 = Point2f::max(&p0, &self.pixel_bounds.p_min);
        p1 = Point2f::min(&p1, &self.pixel_bounds.p_max);

        let mut ifx: Vec<usize> = vec![0usize; (p1.x() - p0.x()) as usize];
        let mut x= p0.x();
        loop {
            if x >= p1.x() { break; }
            let fx = ((x - p_film_discrete.x()) * self.inv_filter_radius.x() * self.filter_table.len() as f32).abs();

            ifx[(x- p0.x()) as usize] = (fx.floor() as usize).min(self.filter_table.len() - 1);

            x += 1.0;
        }

        let mut ify: Vec<usize> = vec![0usize; (p1.y() - p0.y()) as usize];
        let mut y= p0.y();
        loop {
            if y >= p1.y() { break; }
            let fy = ((y - p_film_discrete.y()) * self.inv_filter_radius.y() * self.filter_table.len() as f32).abs();

            ify[(y- p0.y()) as usize] = (fy.floor() as usize).min(self.filter_table.len() - 1);

            y += 1.0;
        }

        y = p0.y();
        'outer: loop {
            if y >= p1.y() { break 'outer; }
            x = p0.x();
            'inner: loop {
                if x >= p1.x() { break 'inner; }

                let offset = ify[(y - p0.y()) as usize] * self.filter_table.len() + ifx[(x - p0.x()) as usize];
                let filter_weight = self.filter_table[offset];

                let pixel = self.get_pixel(&Point2f::init([x, y]));

                pixel.contrib_sum = pixel.contrib_sum + l * sample_weight as f64 * filter_weight as f64;
                pixel.filter_weight_sum += filter_weight;
            }
        }
    }

    pub fn get_pixel(&mut self, p: &Point2f) -> &mut FilmTilePixel {
        let width = self.pixel_bounds.p_max.x() - self.pixel_bounds.p_min.x();
        let offset = (p.x() - self.pixel_bounds.p_min.x()) + (p.y() - self.pixel_bounds.p_min.y()) * width;

        &mut self.pixels[offset as usize]
    }

    pub fn get_pixel_const(&self, p: &Point2f) -> FilmTilePixel {
        let width = self.pixel_bounds.p_max.x() - self.pixel_bounds.p_min.x();
        let offset = (p.x() - self.pixel_bounds.p_min.x()) + (p.y() - self.pixel_bounds.p_min.y()) * width;

        self.pixels[offset as usize].clone()
    }

    pub fn get_pixel_bounds(&self) -> Bounds2f {
        self.pixel_bounds
    }
}