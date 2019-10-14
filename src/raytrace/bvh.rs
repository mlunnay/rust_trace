//! Implementation of a Binding Volume Hierarcy

use super::aabb::AABB;
use super::hittable::{Hittable, HitRecord};
use super::ray::Ray;
use rand::Rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB
}

impl BVHNode {
    pub fn new(bbox: AABB, left: Arc<dyn Hittable>, right: Arc<dyn Hittable>) -> Self {
        BVHNode{bbox, left, right}
    }

    pub fn construct(mut hittable_list: Vec<Box<dyn Hittable>>) -> Arc<BVHNode> {
        let axis = rand::thread_rng().gen_range(0, 3);
        hittable_list.sort_by(|a,b| {
            let left = a.required_bounding_box().min.elements();
            let right = b.required_bounding_box().min.elements();

            left[axis].partial_cmp(&right[axis]).unwrap()
        });
        match hittable_list.len() { 
            0 => panic!("hittable_list is an empty vector"),
            1 => {
                let left: Arc<dyn Hittable> = Arc::from(hittable_list.remove(0));
                let right = Arc::new(Empty{});
                Arc::new(BVHNode{bbox: left.required_bounding_box(), left: Arc::clone(&left), right: right})
            }
            2 => {
                let left: Arc<dyn Hittable> = Arc::from(hittable_list.remove(0));
                let right: Arc<dyn Hittable> = Arc::from(hittable_list.remove(0));
                let bbox = AABB::merge(&left.required_bounding_box(), &right.required_bounding_box());

                Arc::new(BVHNode{ bbox, left, right })
            }
            _ => {
                let mut a = hittable_list;
                let b = a.split_off(a.len() / 2);
                let left = BVHNode::construct(a);
                let right = BVHNode::construct(b);
                let bbox = AABB::merge(&left.required_bounding_box(), &right.required_bounding_box());

                Arc::new(BVHNode{ bbox, left: left, right: right })
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            match (hit_left, hit_right) {
                (None, None) => None,
                (None, Some(hit_record)) => Some(hit_record),
                (Some(hit_record), None) => Some(hit_record),
                (Some(hit_left), Some(hit_right)) => {
                    if hit_left.t < hit_right.t {
                        Some(hit_left)
                    }
                    else {
                        Some(hit_right)
                    }
                }
            }
        }
        else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}

impl std::fmt::Debug for BVHNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BVHNode")
    }
}

struct Empty {}

impl Hittable for Empty {
    fn hit(&self, _r: Ray, _t_min: f64, _t_max: f64) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}