use std::cmp::Ordering;

use super::*;
enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: Aabb,
}

impl BVH {
    pub fn new(mut hit: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> BVH {
        fn box_compare(
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);

                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    std::panic!("No bounding box in BVH node")
                }
            }
        }
        fn axis_range(hit: &Vec<Box<dyn Hittable>>, time0: f64, time1: f64, axis: usize) -> f64 {
            let (min, max) = hit.iter().fold((f64::MAX, f64::MIN), |(bmin, bmax), hit| {
                if let Some(aabb) = hit.bounding_box(time0, time1) {
                    (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f64)> = (0..3)
            .map(|a| (a, axis_range(&hit, time0, time1, a)))
            .collect();
        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let axis = axis_ranges[0].0;
        hit.sort_unstable_by(box_compare(time0, time1, axis));
        let length = hit.len();

        match length {
            0 => std::panic!("no object in the scene"),
            1 => {
                let leaf = hit.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    std::panic!("no bounding box in bvh node")
                }
            }
            _ => {
                let right = BVH::new(hit.drain(length / 2..).collect(), time0, time1);
                let left = BVH::new(hit, time0, time1);
                let bbox = Aabb::surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            match &self.tree {
                BVHNode::Branch { left, right } => {
                    let left = left.hit(&r, t_min, t_max);
                    if let Some(l) = &left {
                        t_max = l.t
                    };
                    let right = right.hit(&r, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
                BVHNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}
