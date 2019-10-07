use std::rc::Rc;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::vec::Vec3;
use crate::raytrace::box_obj::Box as Box_;
use crate::raytrace::material::{Lambertian, DiffuseLight};
use crate::raytrace::texture::ConstantTexture;
use crate::raytrace::camera::Camera;
use crate::raytrace::util::degrees_to_radians;

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

    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.75, 0.75, 0.75)))));

    objects.push(Box::new(Box_::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(555.1, 555.0, 555.0), Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))))))));
    objects.push(Box::new(Box_::new(Vec3::new(-0.1, 0.0, 0.0), Vec3::new(0.0, 555.0, 555.0), Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))))))));
    objects.push(Box::new(Box_::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 555.0, 555.1), white.clone())));
    objects.push(Box::new(Box_::new(Vec3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 555.1, 555.0), white.clone())));
    objects.push(Box::new(Box_::new(Vec3::new(0.0, -0.1, 0.0), Vec3::new(555.0, 0.0, 555.0), white.clone())));
    objects.push(Box::new(Box_::new(Vec3::new(213.0, 554.8, 227.0), Vec3::new(343.0, 554.9, 332.0), Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0))))))));
    

    objects
}