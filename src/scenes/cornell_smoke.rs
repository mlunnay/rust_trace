use std::sync::Arc;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::Vec3;
use crate::raytrace::cuboid::Cuboid;
use crate::raytrace::material::{Lambertian, DiffuseLight, Isotropic};
use crate::raytrace::texture::ConstantTexture;
use crate::raytrace::camera::Camera;
use crate::raytrace::util::degrees_to_radians;
use crate::raytrace::modify::{Translate, RotateY};
use crate::raytrace::constant_medium::ConstantMedium;

pub fn camera(aspect: f64) -> Camera {
    Camera::new(
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        degrees_to_radians(38.0),
        aspect,
        0.0,
        10.0
    )
}

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(0.75, 0.75, 0.75)))));

    objects.push(Box::new(Cuboid::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(555.1, 555.0, 555.0), Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))))))));
    objects.push(Box::new(Cuboid::new(Vec3::new(-0.1, 0.0, 0.0), Vec3::new(0.0, 555.0, 555.0), Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))))))));
    objects.push(Box::new(Cuboid::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 555.0, 555.1), white.clone())));
    objects.push(Box::new(Cuboid::new(Vec3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 555.1, 555.0), white.clone())));
    objects.push(Box::new(Cuboid::new(Vec3::new(0.0, -0.1, 0.0), Vec3::new(555.0, 0.0, 555.0), white.clone())));
    // objects.push(Box::new(Cuboid::new(Vec3::new(113.0, 554.8, 127.0), Vec3::new(443.0, 554.9, 432.0), Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0))))))));
    objects.push(Box::new(Cuboid::new(Vec3::new(213.0, 554.8, 227.0), Vec3::new(343.0, 554.9, 332.0), Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0))))))));

    let cuboid = Arc::new(Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()));
    let obj = Arc::new(Translate::new(Arc::new(RotateY::new(cuboid, degrees_to_radians(-18.0))), Vec3::new(130.0, 0.0, 65.0)));
    objects.push(Box::new(ConstantMedium::new(obj, 0.01, Arc::new(Isotropic::new(Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))))))));

    let cuboid = Arc::new(Cuboid::new(Vec3::new(0.0, 0.0001, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let obj = Arc::new(Translate::new(Arc::new(RotateY::new(cuboid, degrees_to_radians(15.0))), Vec3::new(265.0, 0.0, 295.0)));
    objects.push(Box::new(ConstantMedium::new(obj, 0.01, Arc::new(Isotropic::new(Arc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))))))));

    objects
}