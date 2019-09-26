use super::vec::Vec3;
use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = 2.0 * Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let temp = (-b - f64::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius
                });
            }
            let temp = (-b + f64::sqrt(b * b - a * c)) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord{
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius
                });
            }
        }
        None
    }
}