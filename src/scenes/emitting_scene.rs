use std::sync::Arc;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::Vec3;
use crate::raytrace::sphere::Sphere;
use crate::raytrace::material::{Lambertian, DiffuseLight};
use crate::raytrace::texture::{MarbleTexture, ConstantTexture};
use crate::raytrace::camera::Camera;
use crate::raytrace::util::degrees_to_radians;

pub fn camera(width: u32, height: u32) -> Camera {
    Camera::new(
        Vec3::new(10.0, 2.0, 2.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        width as f64 / height as f64,
        0.0,
        10.0
    )
}

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(Arc::new(MarbleTexture::new(3.0)))))));

    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(Arc::new(MarbleTexture::new(3.0)))))));

    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0))))))));
    objects.push(Box::new(crate::raytrace::cuboid::Cuboid::new(Vec3::new(3.0, 1.0, -2.0), Vec3::new(5.0, 3.0, -2.0001), Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0))))))));

    objects
}