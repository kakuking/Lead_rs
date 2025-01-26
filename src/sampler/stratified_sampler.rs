use crate::common::*;

pub struct StratifiedSampler {
    // Sampler variables
    array_1d_offset: usize,
    array_2d_offset: usize,
    current_pixel: Point2f,
    current_pixel_sample_index: usize,
    samples_1d_array_sizes: Vec<usize>,
    samples_2d_array_sizes: Vec<usize>,
    sample_array_1d: Vec<Vec<f32>>,
    sample_array_2d: Vec<Vec<Point2f>>,
    samples_per_pixel: usize,
    //Pixel sampler variables
    samples_1d: Vec<Vec<f32>>,
    samples_2d: Vec<Vec<Point2f>>,
    current_1d_dim: usize,
    current_2d_dim: usize,
    rng: RNG,
    // stratified sampler variables
    x_pixel_samples: usize,
    y_pixel_samples: usize,
    jitter_samples: bool
}

fn create_stratified_sampler(prop_list: PropertyList) -> LeadObject {
    let sampler = StratifiedSampler::init(prop_list);
    LeadObject::Sampler(Arc::new(sampler))
}

impl Sampler for StratifiedSampler {
    fn array_1d_offset(&self) -> usize { self.array_1d_offset }
    fn array_2d_offset(&self) -> usize { self.array_2d_offset }
    fn current_pixel(&self) -> Point2f { self.current_pixel }
    fn current_pixel_sample_index(&self) -> usize { self.current_pixel_sample_index }
    fn samples_1d_array_sizes(&mut self) -> &mut Vec<usize> { &mut self.samples_1d_array_sizes }
    fn samples_2d_array_sizes(&mut self) -> &mut Vec<usize> { &mut self.samples_2d_array_sizes }
    fn sample_array_1d(&mut self) -> &mut Vec<Vec<f32>> { &mut self.sample_array_1d }
    fn sample_array_2d(&mut self) -> &mut Vec<Vec<Point2f>> { &mut self.sample_array_2d }
    fn samples_per_pixel(&self) -> usize { self.samples_per_pixel }

    fn set_array_1d_offset(&mut self, n: usize) { self.array_1d_offset = n }
    fn set_array_2d_offset(&mut self, n: usize) { self.array_2d_offset = n }
    fn set_current_pixel(&mut self, n: &Point2f) { self.current_pixel = n.clone() }
    fn set_current_pixel_sample_index(&mut self, n: usize) { self.current_pixel_sample_index = n; }
    fn set_samples_per_pixel(&mut self, n: usize) { self.samples_per_pixel = n }

    fn start_pixel(&mut self, p: &Point2f) {
        for i in 0..self.samples_1d.len() {
            Self::stratified_sample_1d(&mut self.samples_1d[i], self.x_pixel_samples * self.y_pixel_samples, &mut self.rng, self.jitter_samples);
            Self::shuffle(&mut self.samples_1d[i], self.x_pixel_samples * self.y_pixel_samples, 1, &mut self.rng);
        }

        for i in 0..self.samples_2d.len() {
            Self::stratified_sample_2d(&mut self.samples_2d[i], self.x_pixel_samples, self.y_pixel_samples, &mut self.rng, self.jitter_samples);
            Self::shuffle(&mut self.samples_2d[i], self.x_pixel_samples * self.y_pixel_samples, 1, &mut self.rng);
        }

        for i in 0..self.samples_1d_array_sizes.len() {
            for j in 0..self.samples_per_pixel {
                let count = self.samples_1d_array_sizes[i];
                Self::stratified_sample_1d(&mut self.sample_array_1d[i][j * count..], count, &mut self.rng, self.jitter_samples);
                Self::shuffle(&mut self.sample_array_1d[i][j * count..], count, 1, &mut self.rng);
            }
        }

        for i in 0..self.samples_2d_array_sizes.len() {
            for j in 0..self.samples_per_pixel {
                let count = self.samples_2d_array_sizes[i];
                Self::latin_hypercube_x(&mut self.sample_array_2d[i][j*count..], count, 2, &mut self.rng);
            }
        }

        // what start_pixel does
        self.set_current_pixel(p);
        self.set_current_pixel_sample_index(0usize);
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);
    }

    fn get_1d(&mut self) -> f32 {
        PixelSampler::get_1d(self)
    }

    fn get_2d(&mut self) -> Point2f {
        PixelSampler::get_2d(self)
    }
}

impl PixelSampler for StratifiedSampler {
    fn samples_1d(&mut self) -> &mut Vec<Vec<f32>> { &mut self.samples_1d }
    fn samples_2d(&mut self) -> &mut Vec<Vec<Point2f>> { &mut self.samples_2d }
    fn current_1d_dim(&mut self) -> &mut usize { &mut self.current_1d_dim }
    fn current_2d_dim(&mut self) -> &mut usize { &mut self.current_2d_dim }
    fn rng(&mut self) -> &mut RNG { &mut self.rng }
}

impl LeadObjectTrait for StratifiedSampler {
    fn init(&mut self, _prop_list: PropertyList) { }
    fn activate(&mut self) { }
    fn add_child(&mut self, _child: &mut LeadObject) { println!("Cannot add a child to Stratified Sampler"); }

    fn to_string(&self) -> String {
        format!(
            "StratifiedSampler[\n  jitter_samples: {},\n  samples_per_pixel: {},\n  x_pixel_samples: {},\n  y_pixel_samples: {},\n  rng_seed: {}\n]",
            if self.jitter_samples { "true" } else { "false" },
            self.samples_per_pixel, self.x_pixel_samples, self.y_pixel_samples, self.rng.seed
        )
    }
}

impl StratifiedSampler {
    pub fn init(property_list: PropertyList) -> Self {
        let x_pixel_samples = property_list.get_int("x_pixel_samples", 1);
        let y_pixel_samples = property_list.get_int("y_pixel_samples", 1);
        let jitter_samples = property_list.get_bool("jitter_samples", true);
        let n_sampled_dimensions = property_list.get_int("n_sampled_dimensions", 1);
        let seed = property_list.get_int("seed", 69) as u64;

        let mut ret = Self {
            array_1d_offset: 0usize,
            array_2d_offset: 0usize,
            current_pixel: Point2f::new(),
            current_pixel_sample_index: 0usize,
            samples_1d_array_sizes: Vec::<usize>::new(),
            samples_2d_array_sizes: Vec::<usize>::new(),
            sample_array_1d: Vec::<Vec<f32>>::new(),
            sample_array_2d: Vec::<Vec<Point2f>>::new(),
            samples_per_pixel: 0usize,
            samples_1d: Vec::<Vec<f32>>::new(),
            samples_2d: Vec::<Vec<Point2f>>::new(),
            current_1d_dim: 0usize,
            current_2d_dim: 0usize,
            rng: RNG::new_seeded(seed),
            x_pixel_samples: x_pixel_samples as usize,
            y_pixel_samples: y_pixel_samples as usize,
            jitter_samples: jitter_samples
        };

        PixelSampler::init(&mut ret, (x_pixel_samples * y_pixel_samples) as usize, n_sampled_dimensions as usize);
        ret
    }

    fn stratified_sample_1d(sample: &mut [f32], n_samples: usize, rng: &mut RNG, jitter: bool) {
        let inv_n_samples = 1.0 / n_samples as f32;
        for i in 0..n_samples {
            let delta = if jitter { rng.uniform_f32() } else { 0.5 };
            sample[i] = ((i as f32 + delta) * inv_n_samples).min(ONE_MINUS_EPSILON);
        }
    }

    fn stratified_sample_2d(sample: &mut [Point2f], nx: usize, ny: usize, rng: &mut RNG, jitter: bool) {
        let dx = 1.0 / nx as f32; let dy = 1.0 / ny as f32;
        let mut i: usize = 0;
        for y in 0..ny {
            for x in 0..nx {
                let jx = if jitter { rng.uniform_f32() } else { 0.5 };
                let jy = if jitter { rng.uniform_f32() } else { 0.5 };

                sample[i][0] = ((x as f32 + jx) * dx).min(ONE_MINUS_EPSILON);
                sample[i][1] = ((y as f32 + jy) * dy).min(ONE_MINUS_EPSILON);
                i += 1;
            }
        }
    }

    fn shuffle<T: Copy>(samp: &mut [T], count: usize, n_dimensions: usize, rng: &mut RNG) {
        for i in 0..count {
            let other = i + rng.uniform_u32_bounded((count - 1) as u32) as usize;
            for j in 0..n_dimensions {
                samp.swap(n_dimensions * i + j, n_dimensions * other + j);
            }
        }
    }

    fn latin_hypercube_x(samples: &mut [Point2f], n_samples: usize, n_dim: usize, rng: &mut RNG) {
        let inv_n_samples = 1.0 / n_samples as f32;
        for i in 0..n_samples {
            for j in 0..n_dim {
                let sj = (i as f32 + rng.uniform_f32()) * inv_n_samples;
                samples[n_dim * i + j][0] = sj.min(ONE_MINUS_EPSILON);
            }
        }

        for i in 0..n_dim {
            for j in 0..n_samples {
                let other = j + rng.uniform_u32_bounded((n_samples - j) as u32) as usize;
                let t = samples[n_dim * j + i][0];  // the x_part
                samples[n_dim * j + i][0] = samples[n_dim * other + i][0];
                samples[n_dim * other + i][0] = t;
            }
        }
    }
}

register_struct!("stratified", create_stratified_sampler);