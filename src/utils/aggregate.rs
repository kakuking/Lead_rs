use crate::common::*;

pub trait Aggregate: Primitive {
    fn primitives(&self) -> &Vec<Arc<dyn Primitive>>;

    fn create(&mut self, primitives: Vec<Arc<dyn Primitive>>, max_primitives_in_node: u32, split_method: SplitMethod);

    fn to_string(&self) -> String;
}