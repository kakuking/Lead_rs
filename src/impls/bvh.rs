use crate::common::*;

#[derive(Debug, Clone, Copy)]
pub enum SplitMethod {
    SAH,
    HLGBH,
    Middle,
    EqualCounts
}

impl SplitMethod {
    pub fn to_string(&self) -> String {
        match self {
            Self::SAH => String::from("Surface Area Heuristic"),
            Self::EqualCounts => String::from("Equal Counts"),
            Self::Middle => String::from("Middle"),
            _ => String::from("Unsupported"),
        }
    }
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

struct LinearBVHNode {
    pub bounds: Bounds3f,
    pub primitives_offset: Option<usize>,   // for leaf
    pub second_child_offset: Option<usize>, // interior
    pub n_primitives: u32,
    pub axis: u32,
}

impl LinearBVHNode {
    pub fn new() -> Self {
        Self {
            bounds: Bounds3f::new(),
            primitives_offset: None,
            second_child_offset: None,
            n_primitives: 0u32,
            axis: 0u32
        }
    }
}

pub struct BVHAccel {
    pub max_primitives_in_node: u32,
    pub split_method: SplitMethod,
    pub primitives: Vec<Arc<dyn Primitive>>,
    nodes: Vec<Arc<LinearBVHNode>>,
}

impl BVHAccel {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            max_primitives_in_node: 0,
            split_method: SplitMethod::SAH,
            nodes: Vec::new()
        }
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
            node.init_leaf(n_primitives, bounds, first_offset as u32);
        } else {
            let mut centroid_bounds = Bounds3f::new();
            for i in start..end {
                centroid_bounds = Bounds3f::union_pt(&centroid_bounds, &primitive_info[i as usize].centroid);
            }
            let dim = centroid_bounds.max_extent() as usize;
            let mut mid = (start + end) / 2;

            if centroid_bounds.p_max[dim] == centroid_bounds.p_min[dim] {
                let first_offset = ordered_primitives.len();
                for i in start..end {
                    let prim_num = primitive_info[i as usize].primitive_number;
                    ordered_primitives.push(self.primitives[prim_num as usize].clone());
                }
                node.init_leaf(n_primitives, bounds, first_offset as u32);
            } else {
                let mut split_method = self.split_method;
                loop {  // in a loop so that it can fall from middle to equal counts if needed
                match split_method {
                SplitMethod::Middle => {
                    let p_mid = (centroid_bounds.p_min[dim] + centroid_bounds.p_max[dim]) / 2.0;

                    mid = {
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
                    mid = (start + end) / 2;
                    primitive_info[start as usize..end as usize]
                        .select_nth_unstable_by(mid as usize, |a, b| a.centroid[dim].partial_cmp(&b.centroid[dim]).unwrap_or(std::cmp::Ordering::Less));
                    break;
                }
                SplitMethod::SAH => {
                    if n_primitives <= 4 {
                        mid = (start + end) / 2;
                        primitive_info[start as usize..end as usize].select_nth_unstable_by((mid - start) as usize, |a, b| {
                            a.centroid[dim].partial_cmp(&b.centroid[dim]).unwrap_or(std::cmp::Ordering::Less)
                        });
                    } else {
                        let n_buckets = 12usize;
                        struct BucketInfo {
                            pub count: f32,
                            pub bounds: Bounds3f
                        }

                        impl BucketInfo {
                            pub fn new() -> Self {
                                Self {
                                    count: 0f32, bounds: Bounds3f::new()
                                }
                            }
                        }

                        let mut buckets: Vec<BucketInfo> = Vec::new();
                        for _ in 0..n_buckets as usize {
                            buckets.push(BucketInfo::new());
                        }
                        for i in start as usize..end as usize {
                            let mut b: usize = n_buckets * (centroid_bounds.offset(&primitive_info[i].centroid)[dim]) as usize;
                            if b == n_buckets {
                                b -= 1usize;
                            }
                            buckets[b].count += 1f32;
                            buckets[b].bounds = Bounds3f::union(&buckets[b].bounds, &primitive_info[i].bounds);
                        }

                        let mut cost: Vec<f32> = Vec::new();
                        for _ in 0..(n_buckets - 1) { cost.push(0f32); }
                        for i in 0..(n_buckets - 1) {
                            let mut b0 = Bounds3f::new();
                            let mut b1 = Bounds3f::new();
                            let mut count_0: f32 = 0f32; let mut count_1: f32 = 0f32;
                            for j in 0..=i {
                                b0 = Bounds3f::union(&b0, &buckets[j].bounds);
                                count_0 += buckets[j].count;
                            }
                            for j in i+1..n_buckets {
                                b1 = Bounds3f::union(&b1, &buckets[j].bounds);
                                count_1 += buckets[j].count;
                            }

                            cost[i] = 0.125 + (count_0 * b0.surface_area() + count_1 * b1.surface_area()) / bounds.surface_area();
                        }

                        let mut min_cost = cost[0]; let mut min_cost_bucket: usize = 0usize;
                        for i in 0..(n_buckets - 1) {
                            if cost[i] < min_cost {
                                min_cost = cost[i];
                                min_cost_bucket = i;
                            }
                        }

                        let leaf_cost = n_primitives as f32;
                        if n_primitives > self.max_primitives_in_node || min_cost < leaf_cost {
                            let mut left = start as usize; let mut right = end as usize;

                            while left < right {
                                while left < right {
                                    let mut b = (n_buckets as f32 * centroid_bounds.offset(&primitive_info[left].centroid)[dim]) as usize;

                                    if b == n_buckets {
                                        b -= 1;
                                    } 
                                    if b > min_cost_bucket {
                                        break;
                                    }
                                    left += 1;
                                }
                                while left < right {
                                    let mut b = (n_buckets as f32 * centroid_bounds.offset(&primitive_info[right - 1].centroid)[dim]) as usize;
                                    if b == n_buckets {
                                        b = n_buckets - 1;
                                    }
                                    if b <= min_cost_bucket {
                                        break;
                                    }
                                    right -= 1;
                                }
                                if left < right {
                                    primitive_info.swap(left, right - 1);
                                }
                                mid = left as u32;
                            }
                        } else {
                            let first_offset = ordered_primitives.len();
                            for i in start as usize..end as usize {
                                let prim_num = primitive_info[i].primitive_number as usize;
                                ordered_primitives.push(self.primitives[prim_num].clone());
                            }

                            node.init_leaf(n_primitives, bounds, first_offset as u32);
                            return Arc::new(node);
                        }
                    }
                    break;
                }
                _ => { split_method = SplitMethod::SAH; }
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

    fn flatten_bvh_tree(&mut self, node: &Arc<BVHBuildNode>, offset: &mut usize) -> usize {    
        let mut linear_node: LinearBVHNode = LinearBVHNode::new();

        linear_node.bounds = node.bounds;

        (*offset) += 1;
        let my_offset = *offset;
        
        if node.n_primitives > 0 {
            linear_node.primitives_offset = Some(node.first_primitive_offset as usize);
            linear_node.n_primitives = node.n_primitives;
        } else {
            linear_node.axis = node.split_axis;
            linear_node.n_primitives = 0u32;

            match &node.children[0] {
                Some(s) => {self.flatten_bvh_tree(s, offset);}
                None => {}
            }
            match &node.children[1] {
                Some(s) => {linear_node.second_child_offset = Some(self.flatten_bvh_tree(s, offset));}
                None => {}
            }
        }

        if self.nodes.len() <= *offset {
            for _ in self.nodes.len()..(*offset) {
                self.nodes.push(Arc::new(LinearBVHNode::new()));
            }
        }
        self.nodes[*offset - 1] = Arc::new(linear_node);

        my_offset
    }

}

impl Aggregate for BVHAccel {
    fn create(&mut self, primitives: Vec<Arc<dyn Primitive>>, max_primitives_in_node: u32, split_method: SplitMethod) {
        self.primitives = primitives;
        self.max_primitives_in_node = if max_primitives_in_node < 255 { max_primitives_in_node } else { 255u32 };
        self.split_method = split_method;

        let mut primitive_info: Vec<BVHPrimitiveInfo> = Vec::new();
        for i in 0..(self.primitives.len()) {
            primitive_info.push(BVHPrimitiveInfo::new(i as u32, self.primitives[i].world_bound()));
        }

        let mut total_nodes = 0u32;
        let mut ordered_primitives: Vec<Arc<dyn Primitive>> = Vec::new();

        let root: Arc<BVHBuildNode> = self.recursive_build(&mut primitive_info, 0, self.primitives.len() as u32, &mut total_nodes, &mut ordered_primitives);

        // TODO - Manipulation of primitives and adding SAH to recursive_build
        self.primitives = ordered_primitives;
        let mut offset: usize = 0usize;

        let max_bvh_nodes = 2 * self.primitives.len() - 1;
        self.nodes.reserve(max_bvh_nodes);

        self.flatten_bvh_tree(&root, &mut offset);
        
        assert_eq!(total_nodes, offset as u32, "Not everything added to BVH!");
    }

    fn primitives(&self) -> &Vec<Arc<dyn Primitive>> {
        &self.primitives
    }
}

impl Primitive for BVHAccel {
    fn compute_scattering_functions(&self, _its: &SurfaceInteraction, _mode: TransportMode, _allow_multiple_lobes: bool) {
        panic!("This should not be called for an aggregate!")
    }

    fn get_area_light(&self) -> Option<Arc<dyn AreaLight>> {
        panic!("This should not be called for an aggregate!")
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        panic!("This should not be called for an aggregate!")
    }

    fn intersect(&self, ray: &Ray, its: &mut SurfaceInteraction) -> bool {
        let mut hit = false;

        let inv_dir = Vector3f::init([1f32 / ray.d.x(), 1f32 / ray.d.y(), 1f32 / ray.d.z()]);
        let dir_is_neg: [i32; 3] = [if inv_dir.x() < 0f32 { 1 } else { 0 }, if inv_dir.y() < 0f32 { 1 } else { 0 }, if inv_dir.z() < 0f32 { 1 } else { 0 }];

        let mut to_visit_offset = 0usize; let mut current_idx = 0usize;
        let mut nodes_to_visit: Vec<usize> = Vec::new();

        loop {
            let node = &self.nodes[current_idx];

            if node.bounds.intersect_inv_p(ray, &inv_dir, dir_is_neg) {
                if node.n_primitives > 0 {
                    if let Some(primitives_offset) = node.primitives_offset{
                        for i in 0..node.n_primitives as usize {
                            if self.primitives[primitives_offset + i].intersect(ray, its) {
                                hit = true;
                            }
                        }
    
                        if to_visit_offset == 0 {
                            break;
                        }
                        to_visit_offset -= 1;
                        current_idx = nodes_to_visit[to_visit_offset];
                    }
                } else if let Some(second_child_offset) = node.second_child_offset {
                    if dir_is_neg[node.axis as usize] == 1 {
                        nodes_to_visit.push(current_idx + 1);
                        to_visit_offset += 1;
                        current_idx = second_child_offset;
                    } else {
                        nodes_to_visit.push(second_child_offset);
                        to_visit_offset += 1;
                        current_idx = current_idx + 1;
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }

                to_visit_offset -= 1;
                current_idx = nodes_to_visit[to_visit_offset];
            }
        }

        hit
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let inv_dir = Vector3f::init([1f32 / ray.d.x(), 1f32 / ray.d.y(), 1f32 / ray.d.z()]);
        let dir_is_neg: [i32; 3] = [if inv_dir.x() < 0f32 { 1 } else { 0 }, if inv_dir.y() < 0f32 { 1 } else { 0 }, if inv_dir.z() < 0f32 { 1 } else { 0 }];

        let mut to_visit_offset = 0usize; let mut current_idx = 0usize;
        let mut nodes_to_visit: Vec<usize> = Vec::new();

        loop {
            let node = &self.nodes[current_idx];

            if node.bounds.intersect_inv_p(ray, &inv_dir, dir_is_neg) {
                if node.n_primitives > 0 {
                    if let Some(primitives_offset) = node.primitives_offset{
                        for i in 0..node.n_primitives as usize {
                            if self.primitives[primitives_offset + i].intersect_p(ray) {
                                return true;
                            }
                        }
    
                        if to_visit_offset == 0 {
                            break;
                        }
                        to_visit_offset -= 1;
                        current_idx = nodes_to_visit[to_visit_offset];
                    }
                } else if let Some(second_child_offset) = node.second_child_offset {
                    if dir_is_neg[node.axis as usize] == 1 {
                        nodes_to_visit.push(current_idx + 1);
                        to_visit_offset += 1;
                        current_idx = second_child_offset;
                    } else {
                        nodes_to_visit.push(second_child_offset);
                        to_visit_offset += 1;
                        current_idx = current_idx + 1;
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }

                to_visit_offset -= 1;
                current_idx = nodes_to_visit[to_visit_offset];
            }
        }

        false
    }

    fn world_bound(&self) -> Bounds3f {
        // TODO - impl this as well
        self.primitives[0 as usize].world_bound()
    }

    fn shape(&self) -> Option<Arc<dyn Shape>> {
        None
    }
}