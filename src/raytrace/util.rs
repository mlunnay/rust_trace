extern crate rand;
use rand::Rng;

use super::vec::Vec3;

pub fn drand48() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2.0 * Vec3{x: drand48(), y: drand48(), z: drand48()} - Vec3{x: 1.0, y: 1.0, z: 1.0};
        p.length_sqared() >= 1.0
    }{}
    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2.0 * Vec3{x: drand48(), y: drand48(), z: 0.0} - Vec3{x: 1.0, y: 1.0, z: 0.0};
        Vec3::dot(&p, &p) >= 1.0
    }{}
    p
}

const PI_DIV_180: f64 = std::f64::consts::PI / 180.0;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    // degrees * std::f64::consts::PI / 180.0
    degrees * PI_DIV_180
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    //radians * 180.0 / std::f64::consts::PI
    radians / PI_DIV_180
}

#[inline(always)]
pub fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= std::f64::EPSILON * a.max(b).max(1.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn drand48() {
        let v: f64 = super::drand48();
        assert!(v > 0.0 && v < 1.0);
    }

    #[test]
    fn degrees_to_radians() {
        assert_eq!(super::degrees_to_radians(180.0), 3.141592653589793);
    }

    #[test]
    fn radians_to_degrees() {
        assert_eq!(super::radians_to_degrees(3.141592653589793), 180.0);
    }
}