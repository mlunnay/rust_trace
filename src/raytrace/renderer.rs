use crate::raytrace::util::drand48;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::ray::Ray;
use crate::raytrace::vec::Vec3;
use crate::raytrace::camera::Camera;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    objects: Box<dyn Hittable>,
    camera: Camera
}

impl Renderer {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, objects: Box<dyn Hittable>) -> Self {
        Renderer{ width, height, samples, objects, camera }
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
            Some(mut rec) => {
                rec.p = rec.p + super::EPSILON * rec.normal;
                // return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
                let emitted = rec.material.emitted(rec.u, rec.v, rec.p);
                let material = &rec.material;
                if depth >= 50 {
                    return emitted;
                }
                else if let Some((scattered, attenuation)) = material.scatter(&ray, &rec) {
                    let ray = Ray::new(scattered.origin + super::EPSILON * rec.normal, scattered.direction);
                    return emitted + attenuation * self.color_from_ray(ray, depth + 1)
                }
                else {
                    emitted
                }
            }
            None => {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
    }
}