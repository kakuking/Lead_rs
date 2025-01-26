use crate::common::*;

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