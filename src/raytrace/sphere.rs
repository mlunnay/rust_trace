use std::rc::Rc;
use super::vec::Vec3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::material::Material;
use super::aabb::AABB;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere{center, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        // the point is "nudged" along the normal to account for precission error to avoid artifacts in reflection
        if discriminant > 0.0 {
            let temp = (-b - f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{
                    t: temp,
                    p: p + super::EPSILON * normal,
                    normal: normal,
                    material: Rc::clone(&self.material)
                });
            }
            let temp = (-b + f64::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord{
                    t: temp,
                    p: p + super::EPSILON * normal,
                    normal: normal,
                    material: Rc::clone(&self.material)
                });
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.center - self.radius, self.center + self.radius))
    }
}