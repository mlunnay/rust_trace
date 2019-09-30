use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x: x, y: y, z: z}
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length_sqared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn elements(&self) -> [f64;3] {
        [self.x, self.y, self.z]
    }

    /// Apply an operation to all elements of this vector, returning the result
    pub fn map(&self, f: fn(f64) -> f64) -> Vec3 {
        Vec3{ x: f(self.x),
            y: f(self.y),
            z: f(self.z)
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3{
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x
        }
    }
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<[f64;3]> for Vec3 {
    fn from(arr: [f64;3]) -> Vec3 { Vec3{x: arr[0], y: arr[1], z: arr[2]} }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(tup: (f64, f64, f64)) -> Vec3 { Vec3{x: tup.0, y: tup.1, z: tup.2} }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self{x: self.x + rhs, y: self.y + rhs, z: self.z + rhs}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self{x: self.x - rhs, y: self.y - rhs, z: self.z - rhs}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Self{x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y,4.0);
        assert_eq!(v.z, 2.0);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.length(), f64::sqrt(21.0));
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.length_sqared(), 21.0);
    }

    #[test]
    fn normalize() {
        let v = Vec3::new(1.0, 4.0, 2.0).normalize();
        let l = f64::sqrt(21.0);
        assert_eq!(v.x, 1.0 / l);
        assert_eq!(v.y, 4.0 / l);
        assert_eq!(v.z, 2.0 / l);
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, -2.0);
    }

    #[test]
    fn add() {
        let v = Vec3::new(1.0, 4.0, 2.0) + Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 9.0);
        assert_eq!(v.z, 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) + 5.0;
        assert_eq!(v2.x, 6.0);
        assert_eq!(v2.y, 9.0);
        assert_eq!(v2.z, 7.0);
    }

    #[test]
    fn sub() {
        let v = Vec3::new(1.0, 4.0, 2.0) - Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x, -2.0);
        assert_eq!(v.y, -1.0);
        assert_eq!(v.z, 3.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) - 1.0;
        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 3.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn mul() {
        let v = Vec3::new(1.0, 4.0, 2.0) * Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 20.0);
        assert_eq!(v.z, -2.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) * 5.0;
        assert_eq!(v2.x, 5.0);
        assert_eq!(v2.y, 20.0);
        assert_eq!(v2.z, 10.0);
    }

    #[test]
    fn div() {
        let v = Vec3::new(1.0, 4.0, 2.0) / Vec3::new(2.0, 4.0, 2.0);
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) / 2.0;
        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v += Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 8.0);
        assert_eq!(v.z, 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 += 2.0;
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 6.0);
        assert_eq!(v2.z, 4.0);
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v -= Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, -2.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 -= 1.0;
        assert_eq!(v2.x, 0.0);
        assert_eq!(v2.y, 3.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v *= Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 16.0);
        assert_eq!(v.z, 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 *= 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 8.0);
        assert_eq!(v2.z, 4.0);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v /= Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.x, 0.5);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 1.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 /= 2.0;
        assert_eq!(v2.x, 0.5);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 1.0);
    }

    #[test]
    fn elements() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        let e = v.elements();
        assert_eq!(e[0], 1.0);
        assert_eq!(e[1], 4.0);
        assert_eq!(e[2], 2.0);
    }

    #[test]
    fn map() {
        let v = Vec3::new(1.0, 4.0, 2.0).map(|v| { v * v });
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 16.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn from() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        let v = Vec3::from((1.0, 2.0, 3.0));
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::dot(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 5.0, 7.0)), 32.0)
    }

    #[test]
    fn cross() {
        let v = Vec3::cross(&Vec3::new(1.0, 2.0, 3.0), &Vec3::new(1.0, 5.0, 7.0));
        assert_eq!(v.x, -1.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.z, 3.0);
    }
}