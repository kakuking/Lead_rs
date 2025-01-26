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