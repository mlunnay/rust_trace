use std::rc::Rc;
use crate::raytrace::util::drand48;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::vec::Vec3;
use crate::raytrace::sphere::Sphere;
use crate::raytrace::material::{Metal, Lambertian, Dielectric};
use crate::raytrace::texture::ConstantTexture;

pub fn generate() -> Vec<Box<dyn Hittable>> {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))))))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = drand48();
            let center = Vec3::new(a as f64 + 0.9 * drand48(), 0.2, b as f64 + 0.9 * drand48());
            if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(drand48() * drand48(), drand48() * drand48(), drand48() * drand48()))))))));
                }
                else if choose_material < 0.95 {
                    objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(Metal::new(Rc::new(ConstantTexture::new(Vec3::new(0.5*(1.0 + drand48()), 0.5*(1.0 + drand48()), 0.5*(1.0 + drand48())))), 0.5*(1.0 + drand48()))))));
                }
                else {
                    objects.push(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
    objects.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1))))))));
    // objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Rc::new(ConstantTexture::new(Vec3::new(0.7, 0.6, 0.5))), 0.0)))));
    objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1))))))));

    objects
}
