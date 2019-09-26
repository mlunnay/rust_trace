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

#[cfg(test)]
mod tests {
    #[test]
    fn drand48() {
        let v: f64 = super::drand48();
        assert!(v > 0.0 && v < 1.0);
    }
}