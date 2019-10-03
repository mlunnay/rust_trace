use std::fs::File;
use std::rc::Rc;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::texture::ImageTexture;
use crate::raytrace::material::Lambertian;
use crate::raytrace::sphere::Sphere;
use crate::raytrace::vec::Vec3;

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let decoder = png::Decoder::new(File::open("earth.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let image = ImageTexture::new(buf, info.width, info.height);

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(Rc::new(image))),
    )));

    objects
}