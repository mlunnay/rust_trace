use super::ray::Ray;
use super::vec::Vec3;
use super::hittable::HitRecord;
use super::util::{random_in_unit_sphere, drand48};

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

pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f64
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f64) -> Self {
        Metal{ albedo, roughness: f64::min(roughness, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(hit_record.p,
            reflected + self.roughness * random_in_unit_sphere());
        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some((scattered, self.albedo))
        }
        else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric{ refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let ni_over_nt: f64;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refracted: Vec3;
        let refracted_prob: f64;
        let mut cosine: f64;
        let scattered: Ray;

        if Vec3::dot(&ray_in.direction, &hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.refractive_index;
            cosine = Vec3::dot(&ray_in.direction, &hit_record.normal) / ray_in.direction.length();
            cosine = f64::sqrt(1.0-self.refractive_index*self.refractive_index*(1.0-cosine*cosine));
        }
        else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -Vec3::dot(&ray_in.direction, &hit_record.normal) / ray_in.direction.length();
        }
        match refract(ray_in.direction, outward_normal, ni_over_nt) {
            Some(v) => {
                refracted = v;
                refracted_prob = schlick(cosine, self.refractive_index);
            }
            None => {
                scattered = Ray::new(hit_record.p, reflected);
                return Some((scattered, attenuation));
            }
        }
        if drand48() < refracted_prob {
            scattered = Ray::new(hit_record.p, reflected);
        }
        else {
            scattered = Ray::new(hit_record.p, refracted);
        }
        Some((scattered, attenuation))
    }
}

pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(&v, &normal) * normal
}

pub fn refract(v: Vec3, normal: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = Vec3::dot(&uv, &normal);
    let descriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if descriminant > 0.0 {
        let refracted = ni_over_nt * (uv - normal * dt) - normal * f64::sqrt(descriminant);
        Some(refracted)
    }
    else {
        None
    }
}

pub fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0-r0)*f64::powi(1.0-cosine, 5)
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