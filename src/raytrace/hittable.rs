use std::rc::Rc;
use super::vec::Vec3;
use super::ray::Ray;
use super::material::Material;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: Rc<dyn Material>) -> Self {
        HitRecord{t, p, normal, material}
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}