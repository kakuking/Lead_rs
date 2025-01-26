use crate::common::*;

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