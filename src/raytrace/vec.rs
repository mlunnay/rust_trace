use core::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use super::util::approx_equal;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub _x: f64,
    pub _y: f64,
    pub _z: f64,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{_x: x, _y: y, _z: z}
    }

    /// Creates an empty Vec3
    #[inline]
    pub fn zero() -> Vec3 {
        Vec3{_x: 0.0, _y: 0.0, _z: 0.0}
    }

    /// Creates a Vector representing the x axis.
    #[inline]
    pub fn unit_x() -> Vec3 {
        Vec3{_x: 1.0, _y: 0.0, _z: 0.0}
    }

    /// Creates a Vector representing the xyaxis.
    #[inline]
    pub fn unit_y() -> Vec3 {
        Vec3{_x: 0.0, _y: 1.0, _z: 0.0}
    }

    /// Creates a Vector representing the z axis.
    #[inline]
    pub fn unit_z() -> Vec3 {
        Vec3{_x: 0.0, _y: 0.0, _z: 1.0}
    }

    #[inline]
    pub fn x(self) -> f64 {
        self._x
    }

    #[inline]
    pub fn y(self) -> f64 {
        self._y
    }

    #[inline]
    pub fn z(self) -> f64 {
        self._z
    }

    #[inline]
    pub fn set_x(&mut self, v: f64) {
        self._x = v;
    }

    #[inline]
    pub fn set_y(&mut self, v: f64) {
        self._x = v;
    }

    #[inline]
    pub fn set_z(&mut self, v: f64) {
        self._x = v;
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self._x * self._x + self._y * self._y + self._z * self._z)
    }

    pub fn length_squared(&self) -> f64 {
        self._x * self._x + self._y * self._y + self._z * self._z
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn elements(&self) -> [f64;3] {
        [self._x, self._y, self._z]
    }

    /// Apply an operation to all elements of this vector, returning the result
    pub fn map(&self, f: fn(f64) -> f64) -> Vec3 {
        Vec3{ _x: f(self._x),
            _y: f(self._y),
            _z: f(self._z)
        }
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1._x * v2._x + v1._y * v2._y + v1._z * v2._z
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3{
            _x: v1._y * v2._z - v1._z * v2._y,
            _y: -(v1._x * v2._z - v1._z * v2._x),
            _z: v1._x * v2._y - v1._y * v2._x
        }
    }

    pub fn is_zero_length(self) -> bool {
        let length = self._x * self._x + self._y * self._y + self._z * self._z;
        approx_equal(length, 0.0)
    }
}

impl core::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "({}, {}, {})", self._x, self._y, self._z)
    }
}

impl From<[f64;3]> for Vec3 {
    fn from(arr: [f64;3]) -> Vec3 { Vec3{_x: arr[0], _y: arr[1], _z: arr[2]} }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(tup: (f64, f64, f64)) -> Vec3 { Vec3{_x: tup.0, _y: tup.1, _z: tup.2} }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{_x: -self._x, _y: -self._y, _z: -self._z}
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self._x == other._x && self._y == other._y && self._z == other._z
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self._x + rhs._x, self._y + rhs._y, self._z + rhs._z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3::new(self._x + rhs, self._y + rhs, self._z + rhs)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Self{_x: self._x + rhs._x, _y: self._y + rhs._y, _z: self._z + rhs._z}
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self{_x: self._x + rhs, _y: self._y + rhs, _z: self._z + rhs}
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self._x - rhs._x, self._y - rhs._y, self._z - rhs._z)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Vec3 {
        Vec3::new(self._x - rhs, self._y - rhs, self._z - rhs)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Self{_x: self._x - rhs._x, _y: self._y - rhs._y, _z: self._z - rhs._z}
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self{_x: self._x - rhs, _y: self._y - rhs, _z: self._z - rhs}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self._x * rhs._x, self._y * rhs._y, self._z * rhs._z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self._x * rhs, self._y * rhs, self._z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            _x: rhs._x * self,
            _y: rhs._y * self,
            _z: rhs._z * self
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self{_x: self._x * rhs._x, _y: self._y * rhs._y, _z: self._z * rhs._z}
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self{_x: self._x * rhs, _y: self._y * rhs, _z: self._z * rhs}
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self._x / rhs._x, self._y / rhs._y, self._z / rhs._z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self._x / rhs, self._y / rhs, self._z / rhs)
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Self{_x: self._x / rhs._x, _y: self._y / rhs._y, _z: self._z / rhs._z}
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self{_x: self._x / rhs, _y: self._y / rhs, _z: self._z / rhs}
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => { &self._x }
            1 => { &self._y }
            2 => { &self._z }
            _ => { panic!("index out of range for Vec3"); }
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => { &mut self._x }
            1 => { &mut self._y }
            2 => { &mut self._z }
            _ => { panic!("index out of range for Vec3"); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v._x, 1.0);
        assert_eq!(v._y,4.0);
        assert_eq!(v._z, 2.0);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.length(), f64::sqrt(21.0));
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.length_squared(), 21.0);
    }

    #[test]
    fn normalize() {
        let v = Vec3::new(1.0, 4.0, 2.0).normalize();
        let l = f64::sqrt(21.0);
        assert_eq!(v._x, 1.0 / l);
        assert_eq!(v._y, 4.0 / l);
        assert_eq!(v._z, 2.0 / l);
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v._x, -1.0);
        assert_eq!(v._y, -4.0);
        assert_eq!(v._z, -2.0);
    }

    #[test]
    fn add() {
        let v = Vec3::new(1.0, 4.0, 2.0) + Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v._x, 4.0);
        assert_eq!(v._y, 9.0);
        assert_eq!(v._z, 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) + 5.0;
        assert_eq!(v2._x, 6.0);
        assert_eq!(v2._y, 9.0);
        assert_eq!(v2._z, 7.0);
    }

    #[test]
    fn sub() {
        let v = Vec3::new(1.0, 4.0, 2.0) - Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v._x, -2.0);
        assert_eq!(v._y, -1.0);
        assert_eq!(v._z, 3.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) - 1.0;
        assert_eq!(v2._x, 0.0);
        assert_eq!(v2._y, 3.0);
        assert_eq!(v2._z, 1.0);
    }

    #[test]
    fn mul() {
        let v = Vec3::new(1.0, 4.0, 2.0) * Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v._x, 3.0);
        assert_eq!(v._y, 20.0);
        assert_eq!(v._z, -2.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) * 5.0;
        assert_eq!(v2._x, 5.0);
        assert_eq!(v2._y, 20.0);
        assert_eq!(v2._z, 10.0);
    }

    #[test]
    fn div() {
        let v = Vec3::new(1.0, 4.0, 2.0) / Vec3::new(2.0, 4.0, 2.0);
        assert_eq!(v._x, 0.5);
        assert_eq!(v._y, 1.0);
        assert_eq!(v._z, 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) / 2.0;
        assert_eq!(v2._x, 0.5);
        assert_eq!(v2._y, 2.0);
        assert_eq!(v2._z, 1.0);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v += Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v._x, 2.0);
        assert_eq!(v._y, 8.0);
        assert_eq!(v._z, 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 += 2.0;
        assert_eq!(v2._x, 3.0);
        assert_eq!(v2._y, 6.0);
        assert_eq!(v2._z, 4.0);
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v -= Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(v._x, 0.0);
        assert_eq!(v._y, 2.0);
        assert_eq!(v._z, -2.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 -= 1.0;
        assert_eq!(v2._x, 0.0);
        assert_eq!(v2._y, 3.0);
        assert_eq!(v2._z, 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v *= Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v._x, 1.0);
        assert_eq!(v._y, 16.0);
        assert_eq!(v._z, 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 *= 2.0;
        assert_eq!(v2._x, 2.0);
        assert_eq!(v2._y, 8.0);
        assert_eq!(v2._z, 4.0);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v /= Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v._x, 0.5);
        assert_eq!(v._y, 2.0);
        assert_eq!(v._z, 1.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 /= 2.0;
        assert_eq!(v2._x, 0.5);
        assert_eq!(v2._y, 2.0);
        assert_eq!(v2._z, 1.0);
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
        assert_eq!(v._x, 1.0);
        assert_eq!(v._y, 16.0);
        assert_eq!(v._z, 4.0);
    }

    #[test]
    fn from() {
        let v = Vec3::from([1.0, 2.0, 3.0]);
        assert_eq!(v._x, 1.0);
        assert_eq!(v._y, 2.0);
        assert_eq!(v._z, 3.0);
        let v = Vec3::from((1.0, 2.0, 3.0));
        assert_eq!(v._x, 1.0);
        assert_eq!(v._y, 2.0);
        assert_eq!(v._z, 3.0);
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::dot(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 5.0, 7.0)), 32.0)
    }

    #[test]
    fn cross() {
        let v = Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 5.0, 7.0));
        assert_eq!(v._x, -1.0);
        assert_eq!(v._y, -4.0);
        assert_eq!(v._z, 3.0);
    }
}