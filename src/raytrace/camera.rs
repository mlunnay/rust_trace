use super::vec::Vec3;
use super::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    // vfov is vertical field of view in radians
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let half_height = f64::tan(vfov / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(&vup, &w);
        let v = Vec3::cross(&w, &u);
        let lower_left_corner = look_from - half_width * u - half_height * v - w;
        Camera{
            origin: look_from,
            lower_left_corner,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * u
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}