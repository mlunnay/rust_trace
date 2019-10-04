use std::rc::Rc;
use std::fs::File;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::vec::Vec3;
use crate::raytrace::box_obj::Box as Box_;
use crate::raytrace::material::Lambertian;
use crate::raytrace::texture::{ConstantTexture, ImageTexture};

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    let decoder = png::Decoder::new(File::open("earth.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let image = ImageTexture::new(buf, info.width, info.height);

    objects.push(Box::new(crate::raytrace::sphere::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))))))));
    // objects.push(Box::new(Box_::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 2.0, 1.0), Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(1.0, 0.0, 0.0))))))));
    objects.push(Box::new(Box_::new(Vec3::new(-1.5, 0.000001, -1.5), Vec3::new(1.5, 3.0, 1.5), Rc::new(Lambertian::new(Rc::new(image))))));

    objects
}