use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub enum SplitMethod {
    SAH,
    HLGBH,
    Middle,
    EqualCounts
}

struct BVHPrimitiveInfo {
    primitive_number: u32,
    bounds: Bounds3f,
    centroid: Point3f
}

impl BVHPrimitiveInfo{
    pub fn new(primitive_number: u32, bounds: Bounds3f) -> Self {
        Self {
            primitive_number,
            centroid: bounds.p_min * 0.5 + bounds.p_max * 0.5,
            bounds,
        }
    }
}

struct  BVHBuildNode {
    bounds: Bounds3f,
    children: [Option<Arc<Self>>; 2],
    split_axis: u32,
    first_primitive_offset: u32,
    n_primitives: u32,
}

impl BVHBuildNode {
    fn new() -> Self {
        Self {
            bounds: Bounds3f::new(),
            children: [None, None],
            split_axis: 0u32,
            first_primitive_offset: 0u32,
            n_primitives: 0u32
        }
    }

    fn init_leaf(&mut self, n: u32, bounds: Bounds3f, first: u32) {
        self.bounds = bounds;
        self.children[0] = None;
        self.children[1] = None;
        self.first_primitive_offset = first;
        self.n_primitives = n;
    }

    fn init_interior(&mut self, axis: u32, c_0: Arc<Self>, c_1: Arc<Self>) {
        self.bounds = Bounds3f::union(&c_0.bounds, &c_1.bounds);
        self.children[0] = Some(c_0);
        self.children[1] = Some(c_1);
        self.split_axis = axis;
        self.n_primitives = 0u32;
    }
}

pub struct BVHAccel {
    pub max_primitives_in_node: u32,
    pub split_method: SplitMethod,
    pub primitives: Vec<Arc<dyn Primitive>>,
}

impl BVHAccel {
    pub fn new(primitives: Vec<Arc<dyn Primitive>>, max_primitives_in_node: u32, split_method: SplitMethod) -> Self {
        let ret: Self = Self{
            primitives,
            max_primitives_in_node: if max_primitives_in_node < 255 { max_primitives_in_node } else { 255u32 },
            split_method
        };

        let mut primitive_info: Vec<BVHPrimitiveInfo> = Vec::new();
        for i in 0..(ret.primitives.len()) {
            primitive_info.push(BVHPrimitiveInfo::new(i as u32, ret.primitives[i].world_bound()));
        }

        let mut total_nodes = 0u32;
        let mut ordered_primitives: Vec<Arc<dyn Primitive>> = Vec::new();

        let _root: Arc<BVHBuildNode> = ret.recursive_build(&mut primitive_info, 0, ret.primitives.len() as u32, &mut total_nodes, &mut ordered_primitives);

        // TODO - Manipulation of primitives and adding SAH to recursive_build

        ret
    }

    fn recursive_build(&self, primitive_info: &mut Vec<BVHPrimitiveInfo>, start: u32, end: u32, total_nodes: &mut u32, ordered_primitives: &mut Vec<Arc<dyn Primitive>>) -> Arc<BVHBuildNode> {
        let mut node: BVHBuildNode = BVHBuildNode::new();
        *total_nodes += 1;
        let mut bounds: Bounds3f = Bounds3f::new();

        for i in start..end{
            bounds = Bounds3f::union(&bounds, &primitive_info[i as usize].bounds);
        }

        let n_primitives = end - start;

        if n_primitives == 1u32 {
            let first_offset = ordered_primitives.len();
            for i in start..end {
                let prim_num = primitive_info[i as usize].primitive_number;
                ordered_primitives.push(self.primitives[prim_num as usize].clone());
            }
            node.init_leaf(first_offset as u32, bounds, n_primitives);
        } else {
            let mut centroid_bounds = Bounds3f::new();
            for i in start..end {
                centroid_bounds = Bounds3f::union_pt(&centroid_bounds, &primitive_info[i as usize].centroid);
            }
            let dim = centroid_bounds.max_extent() as usize;
            let mid = (start + end) / 2;

            if centroid_bounds.p_max[dim] == centroid_bounds.p_min[dim] {
                let first_offset = ordered_primitives.len();
                for i in start..end {
                    let prim_num = primitive_info[i as usize].primitive_number;
                    ordered_primitives.push(self.primitives[prim_num as usize].clone());
                }
                node.init_leaf(first_offset as u32, bounds, n_primitives);
            } else {
                let mut split_method = self.split_method;
                loop {  // in a loop so that it can fall from middle to equal counts if needed
                match split_method {
                SplitMethod::Middle => {
                    let p_mid = (centroid_bounds.p_min[dim] + centroid_bounds.p_max[dim]) / 2.0;

                    let mid = {
                        let mut left = start as usize;
                        let mut right = end as usize;

                        while left < right {
                            // Find the first element on the left that should be on the right
                            while left < right && primitive_info[left].centroid[dim] < p_mid {
                                left += 1;
                            }
                            // Find the first element on the right that should be on the left
                            while left < right && primitive_info[right - 1].centroid[dim] >= p_mid {
                                right -= 1;
                            }
                            // Swap the elements if necessary
                            if left < right {
                                primitive_info.swap(left, right - 1);
                            }
                        }

                        // The `left` pointer now represents the partition point
                        left as u32
                    };

                    if mid != start && mid != end {
                        break;
                    }
                    split_method = SplitMethod::EqualCounts;
                }
                SplitMethod::EqualCounts => {
                    let mid = (start + end) / 2;
                    primitive_info[start as usize..end as usize]
                        .select_nth_unstable_by(mid as usize, |a, b| a.centroid[dim].partial_cmp(&b.centroid[dim]).unwrap_or(std::cmp::Ordering::Less));
                    break;
                }
                _ => {}
                }
                }

                node.init_interior(
                    dim as u32,
                    self.recursive_build(primitive_info, start, mid, total_nodes, ordered_primitives),
                    self.recursive_build(primitive_info, mid, end, total_nodes, ordered_primitives),
                );
            }
        }

        Arc::<BVHBuildNode>::new(node)
    }
}