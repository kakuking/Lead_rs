use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Medium {

}

#[derive(Debug, Clone)]
pub struct MediumInterface {
    pub inside: Arc<Medium>,
    pub outside: Arc<Medium>
}

impl Medium {
    pub fn new() -> Self {
        Self {}
    }
}

impl MediumInterface {
    pub fn new() -> Self {
        Self {
            inside: Arc::<Medium>::new(Medium::new()),
            outside: Arc::<Medium>::new(Medium::new())
        }
    }

    pub fn init(inside: Arc<Medium>, outside: Arc<Medium>) -> Self {
        Self {
            inside,
            outside
        }
    }

    pub fn init_one(medium: Arc<Medium>) -> Self {
        Self {
            inside: medium.clone(),
            outside: medium
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        self.inside == self.outside
    }
}