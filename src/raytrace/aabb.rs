//! Implementation of an Axis Alligned Bounding Box

use super::Vec3;
use super::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB{ min, max }
    }

    pub fn hit(&self, ray: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let inv_d = 1.0 / ray.direction.x();
        let mut t0 = (self.min.x() - ray.origin.x()) * inv_d;
        let mut t1 = (self.max.x() - ray.origin.x()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let inv_d = 1.0 / ray.direction.y();
        t0 = (self.min.y() - ray.origin.y()) * inv_d;
        t1 = (self.max.y() - ray.origin.y()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        let inv_d = 1.0 / ray.direction.z();
        t0 = (self.min.z() - ray.origin.z()) * inv_d;
        t1 = (self.max.z() - ray.origin.z()) * inv_d;
        if inv_d < 0.0 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = if t0 > t_min { t0 } else { t_min };
        t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        true
    }

    pub fn merge(a: &AABB, b: &AABB) -> AABB {
        let min = Vec3::new(
            a.min.x().min(b.min.x()),
            a.min.y().min(b.min.y()),
            a.min.z().min(b.min.z()),
        );
        let max = Vec3::new(
            a.max.x().max(b.max.x()),
            a.max.y().max(b.max.y()),
            a.max.z().max(b.max.z()),
        );
        Self { min, max }
    }

    pub fn get_corners(self) -> [Vec3; 8] {
        let mut vectors: [Vec3; 8] = [Vec3::zero(); 8];
        vectors[0] = Vec3::new(self.min.x(), self.min.y(), self.min.z());
        vectors[1] = Vec3::new(self.min.x(), self.min.y(), self.max.z());
        vectors[2] = Vec3::new(self.min.x(), self.max.y(), self.min.z());
        vectors[3] = Vec3::new(self.min.x(), self.max.y(), self.max.z());
        vectors[4] = Vec3::new(self.max.x(), self.min.y(), self.min.z());
        vectors[5] = Vec3::new(self.max.x(), self.min.y(), self.max.z());
        vectors[6] = Vec3::new(self.max.x(), self.max.y(), self.min.z());
        vectors[7] = Vec3::new(self.max.x(), self.max.y(), self.max.z());
        vectors
    }
}