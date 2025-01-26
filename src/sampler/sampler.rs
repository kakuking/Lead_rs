use crate::common::*;

pub trait Sampler: LeadObjectTrait {
    fn array_1d_offset(&self) -> usize;
    fn array_2d_offset(&self) -> usize;
    fn current_pixel(&self) -> Point2f;
    fn current_pixel_sample_index(&self) -> usize;
    fn samples_1d_array_sizes(&mut self) -> &mut Vec<usize>;
    fn samples_2d_array_sizes(&mut self) -> &mut Vec<usize>;
    fn sample_array_1d(&mut self) -> &mut Vec<Vec<f32>>;
    fn sample_array_2d(&mut self) -> &mut Vec<Vec<Point2f>>;
    fn samples_per_pixel(&self) -> usize;

    fn set_array_1d_offset(&mut self, n: usize);
    fn set_array_2d_offset(&mut self, n: usize);
    fn set_current_pixel(&mut self, n: &Point2f);
    fn set_current_pixel_sample_index(&mut self, n: usize);
    fn set_samples_per_pixel(&mut self, n: usize);

    fn init(&mut self, samples_per_pixel: usize) {
        self.set_samples_per_pixel(samples_per_pixel);
    }
    
    fn start_pixel(&mut self, p: &Point2f) {
        self.set_current_pixel(p);
        self.set_current_pixel_sample_index(0usize);
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);
    }
    fn get_1d(&mut self) -> f32;
    fn get_2d(&mut self) -> Point2f;
    fn get_camera_sample(&mut self, p_raster: &Point2f) -> CameraSample {
        CameraSample {
            p_film: *p_raster + self.get_2d(),
            p_lens: self.get_2d()
        }
    }
    fn request_1d_array(&mut self, n: usize) {
        self.samples_1d_array_sizes().push(n);
        let samples_per_pixel = self.samples_per_pixel();
        self.sample_array_1d().push(vec![0f32; n * samples_per_pixel]);
    }
    fn request_2d_array(&mut self, n: usize) {
        self.samples_2d_array_sizes().push(n);
        let samples_per_pixel = self.samples_per_pixel();
        self.sample_array_2d().push(vec![Point2f::new(); n * samples_per_pixel]);
    }
    fn get_1d_array(&mut self, n: usize) -> &[f32] {
        if self.array_1d_offset() == self.sample_array_1d().len() {
            return &[];
        }

        let start_idx = self.current_pixel_sample_index() * n;
        let end_idx = self.current_pixel_sample_index() * (n + 1);
        let array_1d_offset = self.array_1d_offset();
        self.set_array_1d_offset(self.array_1d_offset() + 1);
        &self.sample_array_1d()[array_1d_offset][start_idx..end_idx]
    }
    fn get_2d_array(&mut self, n: usize) -> &[Point2f] {
        if self.array_2d_offset() == self.sample_array_2d().len() {
            return &[];
        }

        let start_idx = self.current_pixel_sample_index() * n;
        let end_idx = self.current_pixel_sample_index() * (n + 1);
        let array_2d_offset = self.array_2d_offset();
        self.set_array_2d_offset(self.array_2d_offset() + 1);
        &self.sample_array_2d()[array_2d_offset][start_idx..end_idx]
    }
    fn start_next_sample(&mut self) -> bool {
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);
        self.set_current_pixel_sample_index(self.current_pixel_sample_index() + 1);

        self.current_pixel_sample_index() < self.samples_per_pixel()
    }
    // fn clone(&self, _seed: usize) -> Option<Arc<Self>> {
    //     // TODO - figure this out
    //     None
    // }
    fn set_sample_number(&mut self, sample_num: usize) -> bool {
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);

        self.set_current_pixel_sample_index(sample_num);

        self.current_pixel_sample_index() < self.samples_per_pixel()
    }
    
}


pub trait PixelSampler: Sampler {
    fn samples_1d(&mut self) -> &mut Vec<Vec<f32>>;
    fn samples_2d(&mut self) -> &mut Vec<Vec<Point2f>>;
    fn current_1d_dim(&mut self) -> &mut usize;
    fn current_2d_dim(&mut self) -> &mut usize;
    fn rng(&mut self) -> &mut RNG;

    fn init(&mut self, samples_per_pixel: usize, n_sampled_dimensions: usize) {
        Sampler::init(self, samples_per_pixel);
        let samples_per_pixel = self.samples_per_pixel();
        let zero_pt = Point2f::new();
        for _ in 0..n_sampled_dimensions {
            self.samples_1d().push(vec![0f32; samples_per_pixel]);
            self.samples_2d().push(vec![zero_pt; samples_per_pixel]);
        }
    }
    fn start_next_sample(&mut self) -> bool {
        *self.current_1d_dim() = 0usize;
        *self.current_2d_dim() = 0usize;

        Sampler::start_next_sample(self)
    }
    fn set_sample_number(&mut self, sample_num: usize) -> bool {
        *self.current_1d_dim() = 0usize;
        *self.current_2d_dim() = 0usize;

        Sampler::set_sample_number(self, sample_num)
    }
    fn get_1d(&mut self) -> f32 {
        if *self.current_1d_dim() < self.samples_1d().len() {
            *self.current_1d_dim() += 1;
            let cur_1d_dim: usize = *self.current_1d_dim() - 1;
            let cur_pixel_idx: usize = self.current_pixel_sample_index();
            return self.samples_1d()[cur_1d_dim][cur_pixel_idx];
        }
        self.rng().uniform_f32()
    }
    fn get_2d(&mut self) -> Point2f {
        if *self.current_2d_dim() < self.samples_2d().len() {
            *self.current_2d_dim() += 1;
            let cur_2d_dim: usize = *self.current_2d_dim() - 1;
            let cur_pixel_idx: usize = self.current_pixel_sample_index();
            return self.samples_2d()[cur_2d_dim][cur_pixel_idx];
        }
        
        Point2f::init([self.rng().uniform_f32(), self.rng().uniform_f32()])
    }
}

pub trait GlobalSampler: Sampler {
    fn dimension(&mut self) -> &mut usize;
    fn interval_sample_idx(&mut self) -> &mut usize;
    fn array_start_dim(&mut self) -> usize { 5usize }
    fn array_end_dim(&mut self) -> &mut usize;

    fn new(samples_per_pixel: usize) -> Self;
    fn start_next_sample(&mut self) -> bool {
        *self.dimension() = 0usize;
        *self.interval_sample_idx() = self.get_idx_for_sample(self.current_pixel_sample_index() + 1);

        Sampler::start_next_sample(self)
    }
    fn start_pixel(&mut self, p: &Point2f) {
        Sampler::start_pixel(self, p);
        *self.dimension() = 0usize;
        *self.interval_sample_idx() = self.get_idx_for_sample(0usize);
        *self.array_end_dim() = self.array_start_dim() + self.sample_array_1d().len() + 2 * self.sample_array_2d().len();

        for i in 0..self.samples_1d_array_sizes().len() {
            let n_samples = self.samples_1d_array_sizes()[i] * self.samples_per_pixel();

            for j in 0..n_samples {
                let idx = self.get_idx_for_sample(j);
                let start_dim = self.array_start_dim();
                self.sample_array_1d()[i][j] = self.sample_dimension(idx, start_dim + i);
            }
        }
    }
    fn set_sample_number(&mut self, sample_num: usize) -> bool {
        *self.dimension() = 0usize;
        *self.interval_sample_idx() = self.get_idx_for_sample(sample_num);

        Sampler::set_sample_number(self, sample_num)
    }
    fn get_1d(&mut self) -> f32 {
        let dimension: usize = *self.dimension();
        if dimension >= self.array_start_dim() && dimension < *self.array_end_dim() {
            *self.dimension() = *self.array_end_dim();
        }
        *self.dimension() += 1usize;
        let interval_sample_idx = *self.interval_sample_idx();
        self.sample_dimension(interval_sample_idx, dimension)
    }
    fn get_2d(&mut self) -> Point2f {
        let dimension: usize = *self.dimension();
        if dimension + 1 >= self.array_start_dim() && dimension < *self.array_end_dim() {
            *self.dimension() = *self.array_end_dim();
        }
        *self.dimension() += 2usize;
        let interval_sample_idx = *self.interval_sample_idx();
        let t1 = self.sample_dimension(interval_sample_idx, dimension);
        let t2 = self.sample_dimension(interval_sample_idx, dimension + 1);

        Point2f::init([t1, t2])
    }
    fn get_idx_for_sample(&self, sample_num: usize) -> usize;
    fn sample_dimension(&self, idx: usize, dimension: usize) -> f32;
}