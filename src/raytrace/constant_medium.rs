use super::util::drand48;
use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::Vec3;
use std::sync::Arc;
use super::material::Material;
use super::ray::Ray;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, phase_function: Arc<dyn Material>) -> ConstantMedium {
        ConstantMedium{boundary, density, phase_function}
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self.boundary.hit(r, t_min, t_max) {
            Some(mut rec1) => {
                match self.boundary.hit(r, rec1.t + super::EPSILON, t_max) {
                    Some(mut rec2) => {
                        // eprintln!("rec1.t: {:?} rec2.t: {:?}", rec1.t, rec2.t);
                        if rec1.t < t_min {
                            rec1.t = t_min;
                        }
                        if rec2.t > t_max {
                            rec2.t = t_max;
                        }
                        if rec1.t >= rec2.t {
                            return None;
                        }
                        if rec1.t < 0.0 {
                            rec1.t = 0.0;
                        }

                        let ray_length = r.direction.length();
                        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                        let hit_distance = -(1.0 / self.density) * drand48().ln();

                        // eprintln!("hit_distance: {} distance_inside_boundary: {}", hit_distance, distance_inside_boundary);
                        if hit_distance < distance_inside_boundary {
                            // eprintln!("inside boundary");
                            let t = rec1.t + hit_distance / ray_length;
                            let p = r.point_at_parameter(t);
                            let normal = Vec3::new(1.0, 0.0, 0.0);  // arbitrary
                            Some(HitRecord::new(
                                t,
                                p,
                                normal,
                                self.phase_function.clone(),
                                0.0,
                                0.0
                            ))
                        }
                        else {
                            // eprintln!("hit but not inside boundary");
                            None
                        }
                    }
                    None => None
                }
            },
            None => None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}