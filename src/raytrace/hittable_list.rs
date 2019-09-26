use super::ray::Ray;
use super::hittable::{Hittable, HitRecord};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList{
            objects: Vec::new()
        }
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
}