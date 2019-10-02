use std::rc::Rc;
use super::vec::Vec3;
use super::ray::Ray;
use super::material::Material;
use super::aabb::AABB;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub u: f64,
    pub v: f64
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3, material: Rc<dyn Material>, u: f64, v: f64) -> Self {
        HitRecord{t, p, normal, material, u, v}
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
    fn required_bounding_box(&self) -> AABB {
        self.bounding_box().expect("No Bounding Box Found")
    }
}