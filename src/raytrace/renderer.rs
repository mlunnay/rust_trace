use crate::raytrace::util::drand48;
use crate::raytrace::hittable::Hittable;
use crate::raytrace::ray::Ray;
use crate::raytrace::Vec3;
use crate::raytrace::camera::Camera;
use std::sync::Arc;

pub type BgFunc = Box<dyn Fn(Ray) -> Vec3 + Send + Sync>;

pub trait Background: Send + Sync {
    fn get(&self, ray: Ray) -> Vec3;
}

pub struct GradientBackground {}

impl GradientBackground {}

impl Background for GradientBackground {
    fn get(&self, ray: Ray) -> Vec3 {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
    }
}

pub struct Renderer  {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    objects: Arc<dyn Hittable>,
    camera: Camera,
    background: Option<Box<dyn Background>>
}

impl Renderer {
    pub fn new(width: u32, height: u32, samples: u32, camera: Camera, objects: Arc<dyn Hittable>, background: Option<Box<dyn Background>>) -> Self {
        Renderer{ width, height, samples, objects, camera, background }
    }

    pub fn color_at(&self, u: f64, v: f64) -> Vec3 {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _s in 0..self.samples {
            let u = (u + drand48()) / self.width as f64;
            let v = (v + drand48()) / self.height as f64;
            let r = self.camera.get_ray(u, v);
            col += self.color_from_ray(r, 0);
        }
        col /= self.samples as f64;
        col
    }

    fn color_from_ray(&self, ray: Ray, depth: u32) -> Vec3 {
        match self.objects.hit(ray, 0.0, std::f64::MAX) {
            Some(rec) => {
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
                match &self.background {
                    Some(bg) => bg.get(ray), 
                    None => Vec3::new(0.0, 0.0, 0.0)
                }
            }
        }
    }
}