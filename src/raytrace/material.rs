use super::ray::Ray;
use super::vec::Vec3;
use super::hittable::HitRecord;
use super::util::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let target: Vec3 = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        Some((scattered, self.albedo))
    }
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(&v, &normal) * normal
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn reflection() {
        let v = reflect(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 0.0);
    }
}