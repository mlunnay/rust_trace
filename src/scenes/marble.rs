use std::sync::Arc;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::vec::Vec3;
use crate::raytrace::sphere::Sphere;
use crate::raytrace::material::Lambertian;
use crate::raytrace::texture::MarbleTexture;

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(Arc::new(MarbleTexture::new(3.0)))))));

    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(Arc::new(MarbleTexture::new(3.0)))))));

    objects
}
