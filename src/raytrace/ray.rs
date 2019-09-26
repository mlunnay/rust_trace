use super::vec::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ray = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(4.0,4.0,4.0));
        assert_eq!(ray.origin.x, 0.0);
        assert_eq!(ray.origin.y, 0.0);
        assert_eq!(ray.origin.z, 0.0);
        assert_eq!(ray.direction.x, 4.0);
        assert_eq!(ray.direction.y, 4.0);
        assert_eq!(ray.direction.z, 4.0);
    }

    #[test]
    fn point_at_parameter() {
        let ray = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(4.0,4.0,4.0));
        let p = ray.point_at_parameter(0.0);
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
        let p = ray.point_at_parameter(0.5);
        assert_eq!(p.x, 2.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 2.0);
        let p = ray.point_at_parameter(1.0);
        assert_eq!(p.x, 4.0);
        assert_eq!(p.y, 4.0);
        assert_eq!(p.z, 4.0);
    }
}
