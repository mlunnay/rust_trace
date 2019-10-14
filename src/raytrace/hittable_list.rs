use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};
use super::aabb::AABB;
use super::Vec3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList{
            objects: Vec::new()
        }
    }

    pub fn from_vec(v: Vec<Box<dyn Hittable>>) -> HittableList {
        let objects = Vec::from(v);
        HittableList{objects}
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest: f64 = t_max;
        for object in &self.objects {
            let result = object.hit(r, t_min, closest);
            match result {
                Some(r) => {
                    closest = r.t;
                    hit = Some(r);
                }
                None => {}
            }
        }
        hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.objects.len() == 1 { return None; }
        let mut result = AABB::new(Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX), Vec3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN));
        for object in &self.objects {
            if let Some(obj) = object.bounding_box() {
                result = AABB::merge(&result, &obj);
            }
            else {
                return None;
            }
        }

        Some(result)
    }
}