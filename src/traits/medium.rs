use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Medium {

}

#[derive(Debug, Clone)]
pub struct MediumInterface {
    pub inside: Rc<Medium>,
    pub outside: Rc<Medium>
}

impl Medium {
    pub fn new() -> Self {
        Self {}
    }
}

impl MediumInterface {
    pub fn new() -> Self {
        Self {
            inside: Rc::<Medium>::new(Medium::new()),
            outside: Rc::<Medium>::new(Medium::new())
        }
    }

    pub fn is_homogeneous(&self) -> bool {
        self.inside == self.outside
    }
}