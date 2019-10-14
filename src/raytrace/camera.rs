use super::Vec3;
use super::ray::Ray;
use super::util::random_in_unit_disk;

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64
}

impl Camera {
    // vfov is vertical field of view in radians
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64, apeture: f64, focus_distance: f64) -> Self {
        let half_height = f64::tan(vfov / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(vup, w);
        let v = Vec3::cross(w, u);
        let lower_left_corner = look_from - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w;
        Camera{
            origin: look_from,
            lower_left_corner,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            u,
            v,
            _w: w,
            lens_radius: apeture / 2.0
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd + self.v * rd;
        Ray::new(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}