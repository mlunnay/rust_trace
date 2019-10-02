use std::rc::Rc;
use crate::raytrace::util::drand48;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::ray::Ray;
use crate::raytrace::vec::Vec3;
use crate::raytrace::camera::Camera;
use crate::raytrace::sphere::Sphere;
use crate::raytrace::material::{Metal, Lambertian, Dielectric};
use crate::raytrace::bvh::BVHNode;
use crate::raytrace::texture::{ConstantTexture, CheckerTexture};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    objects: BVHNode,
    camera: Camera
}

impl Scene {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera) -> Self {
        let objects = Rc::try_unwrap(BVHNode::construct(Scene::generate())).unwrap();
        Scene{ width, height, samples, objects: objects, camera }
    }

    pub fn generate() -> Vec<Box<dyn Hittable>> {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))), Rc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))))))))));

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
        objects.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Rc::new(ConstantTexture::new(Vec3::new(0.7, 0.6, 0.5))), 0.0)))));

        objects
    }

    pub fn color_at(&self, u: f64, v: f64) -> Vec3 {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _s in 0..self.samples {
            let u = u + drand48() / self.width as f64;
            let v = v + drand48() / self.height as f64;
            let r = self.camera.get_ray(u, v);
            col += self.color_from_ray(r, 0);
        }
        col /= self.samples as f64;
        col.x = f64::sqrt(col.x);
        col.y = f64::sqrt(col.y);
        col.z = f64::sqrt(col.z);
        col
    }

    fn color_from_ray(&self, ray: Ray, depth: u32) -> Vec3 {
        match self.objects.hit(ray, 0.0, std::f64::MAX) {
            Some(rec) => {
                //return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
                if depth >= 50 {
                    Vec3::new(0.0, 0.0, 0.0)
                }
                else if let Some((scattered, attenuation)) = rec.material.scatter(&ray, &rec) {
                    let ray = Ray::new(scattered.origin, scattered.direction);
                    attenuation * self.color_from_ray(ray, depth + 1)
                }
                else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            }
            None => {
                let unit_direction = ray.direction.normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }
}