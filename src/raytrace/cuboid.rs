use super::vec::Vec3;
use super::ray::Ray;
use super::aabb::AABB;
use super::hittable::*;
use super::material::Material;
use std::sync::Arc;

pub struct Cuboid {
    min: Vec3,
    max: Vec3,
    material: Arc::<dyn Material>
}

impl Cuboid {
    pub fn new(min: Vec3, max: Vec3, material: Arc::<dyn Material>) -> Self {
        Self{min, max, material}
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: Ray, bt_min: f64, bt_max: f64) -> Option<HitRecord> {
        let inv_d = 1.0 / ray.direction.x;
        let t0 = (self.min.x - ray.origin.x) * inv_d;
        let t1 = (self.max.x - ray.origin.x) * inv_d;
        // let mut swaped = false;
        // if inv_d < 0.0 {
        //     std::mem::swap(&mut t0, &mut t1);
        //     swaped = true;
        // }
        // t_min = if t0 > t_min { t0 } else { t_min };
        // t_max = if t1 < t_max { t1 } else { t_max };
        // if t_max <= t_min {
        //     return None;
        // }

        let inv_d = 1.0 / ray.direction.y;
        let t2 = (self.min.y - ray.origin.y) * inv_d;
        let t3 = (self.max.y - ray.origin.y) * inv_d;
        // if inv_d < 0.0 {
        //     std::mem::swap(&mut t2, &mut t3);
        //     swaped = true;
        // }
        // t_min = if t2 > t_min { t2 } else { t_min };
        // t_max = if t3 < t_max { t3 } else { t_max };
        // if t_max <= t_min {
        //     return None;
        // }

        let inv_d = 1.0 / ray.direction.z;
        let t4 = (self.min.z - ray.origin.z) * inv_d;
        let t5 = (self.max.z - ray.origin.z) * inv_d;
        // if inv_d < 0.0 {
        //     std::mem::swap(&mut t4, &mut t5);
        //     swaped = true;
        // }
        // t_min = if t4 > t_min { t4 } else { t_min };
        // t_max = if t5 < t_max { t5 } else { t_max };
        // if t_max <= t_min {
        //     return None;
        // }

        let t_min = t0.min(t1).max(t2.min(t3)).max(t4.min(t5));
        let t_max = t0.max(t1).min(t2.max(t3)).min(t4.max(t5));

        // Cuboid is completely behind camera or complete miss
        if t_max < 0.0 || t_min > t_max {
            return None;
        }
        
        let t = if t_min < bt_min { t_max } else { t_min };
        if t > bt_max {
            return None;
        }
        let u: f64;
        let v: f64;
        let point = ray.point_at_parameter(t);
        let normal = if t == t0 { 
                u = (point.z - self.min.z) / (self.max.z - self.min.z);
                v = (point.y - self.min.y) / (self.max.y - self.min.y);
                Vec3::new(-1.0, 0.0, 0.0) 
            } 
            else if t == t1 {
                u = 1.0 - (point.z - self.min.z) / (self.max.z - self.min.z);
                v = (point.y - self.min.y) / (self.max.y - self.min.y);
                Vec3::new(1.0, 0.0, 0.0) 
            }
            else if t == t2 { 
                u = (point.z - self.min.z) / (self.max.z - self.min.z);
                v = (point.x - self.min.x) / (self.max.x - self.min.x);
                Vec3::new(0.0, -1.0, 0.0)
            }
            else if t == t3 { 
                u = 1.0 - (point.z - self.min.z) / (self.max.z - self.min.z);
                v = (point.x - self.min.x) / (self.max.x - self.min.x);
                Vec3::new(0.0, 1.0, 0.0)
            }
            else if t == t4 { 
                u = (point.x - self.min.x) / (self.max.x - self.min.x);
                v = (point.y - self.min.y) / (self.max.y - self.min.y);
                Vec3::new(0.0, 0.0, -1.0)
            }
            else if t == t5 { 
                u = 1.0 - (point.x - self.min.x) / (self.max.x - self.min.x);
                v = (point.y - self.min.y) / (self.max.y - self.min.y);
                Vec3::new(0.0, 0.0, 1.0)
            }
            else { panic!("t should match one of the previous values") };
        
        Some(HitRecord::new(
            t,
            ray.point_at_parameter(t),
            normal,
            Arc::clone(&self.material),
            u,
            v
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB{min: self.min, max: self.max})
    }
}