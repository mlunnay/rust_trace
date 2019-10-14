#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use core::mem::MaybeUninit;

use std::fmt;
use std::ops::*;
use super::util::{Align16, approx_equal};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vec3(pub(crate) __m256d);

impl Vec3 {
    /// Construct a new Vec3 from x, y and z values.
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        unsafe { Vec3(_mm256_set_pd(0.0, z, y, x)) }
    }

    /// Creates a new Vec3 with all values set to 0.
    #[inline]
    pub fn zero() -> Vec3 {
        unsafe { Vec3(_mm256_setzero_pd()) }
    }

    /// Creates a new Vec3 with all values set to 1.
    #[inline]
    pub fn one() -> Vec3 {
        unsafe { Vec3(_mm256_set1_pd(1.0)) }
    }

    #[inline]
    /// Creates a new Vec3 with the values[1.0,0.0,0.0].
    pub fn unit_x() -> Vec3 {
        unsafe { Vec3(_mm256_set_pd(0.0, 0.0, 0.0, 1.0)) }
    }

    /// Creates a new Vec3 with the values[0.0,1.0,0.0].
    #[inline]
    pub fn unit_y() -> Vec3 {
        unsafe { Vec3(_mm256_set_pd(0.0, 0.0, 1.0, 0.0)) }
    }

    /// Creates a new Vec3 with the values[0.0,0.0,1.0].
    #[inline]
    pub fn unit_z() -> Vec3 {
        unsafe { Vec3(_mm256_set_pd(0.0, 1.0, 0.0, 0.0)) }
    }

    /// Creates a new Vec3 with the values all set to v. 
    #[inline]
    pub fn splat(v: f64) -> Vec3 {
        unsafe { Vec3(_mm256_set1_pd(v)) }
    }

    /// Returns the value of x.
    #[inline]
    pub fn x(self) -> f64 {
        unsafe { _mm256_cvtsd_f64(self.0) }
    }

    /// Returns the value of y.
    #[inline]
    pub fn y(self) -> f64 {
        unsafe { _mm256_cvtsd_f64(_mm256_permute_pd(self.0, 0b01)) }
    }

    /// Returns the value of z.
    #[inline]
    pub fn z(self) -> f64 {
        unsafe { _mm256_cvtsd_f64(_mm256_permute4x64_pd(self.0, 2)) }
    }

    /// Sets the value of x.
    #[inline]
    pub fn set_x(&mut self, v: f64) {
        unsafe {
            self.0 = _mm256_blend_pd(self.0, _mm256_set1_pd(v), 0b1);
        }
    }

    /// Sets the value of y.
    #[inline]
    pub fn set_y(&mut self, v: f64) {
        unsafe {
            self.0 = _mm256_blend_pd(self.0, _mm256_set1_pd(v), 0b10);
        }
    }

    /// Sets the value of z.
    #[inline]
    pub fn set_z(&mut self, v: f64) {
        unsafe {
            self.0 = _mm256_blend_pd(self.0, _mm256_set1_pd(v), 0b100);
        }
    }

    /// Calculate the length of this vector.
    #[inline]
    pub fn length(self) -> f64 {
        unsafe {
            let x2_y2_z2_w2 = _mm256_mul_pd(self.0, self.0);
            _mm256_cvtsd_f64(
                _mm256_sqrt_pd(
                    _mm256_add_pd(
                        _mm256_add_pd(
                            x2_y2_z2_w2,
                            _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_01)
                        ),
                        _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_10)
                    )
                )
            )
        }
    }

    /// Calculate the length of this vector.
    #[inline]
    pub fn length_squared(self) -> f64 {
        unsafe {
            let x2_y2_z2_w2 = _mm256_mul_pd(self.0, self.0);
            _mm256_cvtsd_f64(
                _mm256_add_pd(
                    _mm256_add_pd(
                        x2_y2_z2_w2,
                        _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_01)
                    ),
                    _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_10)
                )
            )
        }
    }

    /// Returns the vector normalized to unit length.
    #[inline]
    pub fn normalize(self) -> Vec3 {
        unsafe {
            let x2_y2_z2_w2 = _mm256_mul_pd(self.0, self.0);
            Vec3(
                _mm256_div_pd(
                    self.0,
                    _mm256_permute4x64_pd(
                        _mm256_sqrt_pd(
                            _mm256_add_pd(
                                _mm256_add_pd(
                                    x2_y2_z2_w2,
                                    _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_01)
                                ),
                                _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_10)
                            )
                        ),
                    0b00_00_00_00)
                )
            )
        }
    } 
    
    /// Calculate the dot product of this and another vector.
    #[inline]
    pub fn dot(self, other: Vec3) -> f64 {
        unsafe {
            let x2_y2_z2_w2 = _mm256_mul_pd(self.0, other.0);
            _mm256_cvtsd_f64(
                _mm256_add_pd(
                    _mm256_add_pd(
                        x2_y2_z2_w2,
                        _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_01)
                    ),
                    _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_10)
                )
            )
        }
    }

    /// Calculate the cross product of this and anther vector.
    #[inline]
    pub fn cross(self, other: Vec3) -> Vec3 {
        unsafe {
            Vec3(
            _mm256_permute4x64_pd(
                _mm256_sub_pd(
                    _mm256_mul_pd(self.0, _mm256_permute4x64_pd(other.0, 0b11_00_10_01)),
                    _mm256_mul_pd(other.0, _mm256_permute4x64_pd(self.0, 0b11_00_10_01))
                ),
                0b11_00_10_01
            )
        )}
    }

    /// Returns true if the length of the vector is approximatly 0.
    #[inline]
    pub fn is_zero_length(self) -> bool {
        unsafe {
            let x2_y2_z2_w2 = _mm256_mul_pd(self.0, self.0);
            let len = _mm256_cvtsd_f64(
                _mm256_add_pd(
                    _mm256_add_pd(
                        x2_y2_z2_w2,
                        _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_01)
                    ),
                    _mm256_permute4x64_pd(x2_y2_z2_w2, 0b11_11_11_10)
                )
            );
            approx_equal(len, 0.0)
        }
    }

    /// Return the elements of this Vector as an array [x,y,z].
    #[inline]
    pub fn elements(&self) -> [f64; 3] {
        let (x, y, z) = (*self).into();
        [x,y,z]
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        f.debug_tuple("Vec3").field(&x).field(&y).field(&z).finish()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        write!(f, "({}, {}, {})", x, y, z)
    }
}


impl From<__m256d> for Vec3 {
    #[inline]
    fn from(t: __m256d) -> Self {
        Self(t)
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    #[inline]
    fn from(t: (f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<Vec3> for (f64, f64, f64) {
    #[inline]
    fn from(v: Vec3) -> Self {
        let mut out: MaybeUninit<Align16<(f64, f64, f64)>> = MaybeUninit::uninit();
        unsafe {
            // out is 16 bytes in size due to alignment
            _mm256_store_pd(out.as_mut_ptr() as *mut f64, v.0);
            out.assume_init().0
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        unsafe {
            Vec3(_mm256_sub_pd(_mm256_setzero_pd(), self.0))
        }
    }
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Vec3::zero()
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Vec3) -> bool {
        unsafe {
            _mm256_movemask_pd(
                _mm256_cmp_pd(self.0, other.0, 0b0)
            ) & 0b111 == 0b111
        }
    }
}

// TODO: look at implementing PartialOrd

impl Div<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_div_pd(self.0, rhs.0)) }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm256_div_pd(self.0, rhs.0);
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        unsafe { Self(_mm256_div_pd(self.0, _mm256_set1_pd(rhs))) }
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        unsafe { self.0 = _mm256_div_pd(self.0, _mm256_set1_pd(rhs)) }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_mul_pd(self.0, rhs.0)) }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm256_mul_pd(self.0, rhs.0);
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        unsafe { Self(_mm256_mul_pd(self.0, _mm256_set1_pd(rhs))) }
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        unsafe { self.0 = _mm256_mul_pd(self.0, _mm256_set1_pd(rhs)) }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm256_mul_pd(rhs.0, _mm256_set1_pd(self))) }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_add_pd(self.0, rhs.0)) }
    }
}

impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm256_add_pd(self.0, rhs.0);
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        unsafe { Self(_mm256_add_pd(self.0, _mm256_set1_pd(rhs))) }
    }
}

impl AddAssign<f64> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        unsafe { self.0 = _mm256_add_pd(self.0, _mm256_set1_pd(rhs)) }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_sub_pd(self.0, rhs.0)) }
    }
}

impl SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        unsafe {
            self.0 = _mm256_sub_pd(self.0, rhs.0);
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        unsafe { Self(_mm256_sub_pd(self.0, _mm256_set1_pd(rhs))) }
    }
}

impl SubAssign<f64> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        unsafe { self.0 = _mm256_sub_pd(self.0, _mm256_set1_pd(rhs)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_values() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn set_values() {
        let mut v = Vec3::zero();
        v.set_x(1.0);
        v.set_y(2.0);
        v.set_z(3.0);
        eprintln!("{:?}", v);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn dot() {
        let v = Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(v, 32.0);
    }

    #[test]
    fn cross() {
        let v = Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 5.0, 7.0));
        assert_eq!(v.x(), -1.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.length_squared(), 21.0);
        assert_eq!(v.length(), f64::sqrt(21.0));
    }

    #[test]
    fn normalize() {
        let v = Vec3::new(1.0, 4.0, 2.0).normalize();
        let l = f64::sqrt(21.0);
        assert_eq!(v.x(), 1.0 / l);
        assert_eq!(v.y(), 4.0 / l);
        assert_eq!(v.z(), 2.0 / l);
    }

    #[test]
    fn eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1, v1);
        assert_ne!(v1, Vec3::new(1.0,1.0,std::f64::NAN));
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x(), -1.0);
        assert_eq!(v.y(), -4.0);
        assert_eq!(v.z(), -2.0);
    }

    #[test]
    fn add() {
        let v = Vec3::new(1.0, 4.0, 2.0) + Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x(), 4.0);
        assert_eq!(v.y(), 9.0);
        assert_eq!(v.z(), 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) + 5.0;
        assert_eq!(v2.x(), 6.0);
        assert_eq!(v2.y(), 9.0);
        assert_eq!(v2.z(), 7.0);
    }

    #[test]
    fn sub() {
        let v = Vec3::new(1.0, 4.0, 2.0) - Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x(), -2.0);
        assert_eq!(v.y(), -1.0);
        assert_eq!(v.z(), 3.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) - 1.0;
        assert_eq!(v2.x(), 0.0);
        assert_eq!(v2.y(), 3.0);
        assert_eq!(v2.z(), 1.0);
    }

    #[test]
    fn mul() {
        let v = Vec3::new(1.0, 4.0, 2.0) * Vec3::new(3.0, 5.0, -1.0);
        assert_eq!(v.x(), 3.0);
        assert_eq!(v.y(), 20.0);
        assert_eq!(v.z(), -2.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) * 5.0;
        assert_eq!(v2.x(), 5.0);
        assert_eq!(v2.y(), 20.0);
        assert_eq!(v2.z(), 10.0);
    }

    #[test]
    fn div() {
        let v = Vec3::new(1.0, 4.0, 2.0) / Vec3::new(2.0, 4.0, 2.0);
        assert_eq!(v.x(), 0.5);
        assert_eq!(v.y(), 1.0);
        assert_eq!(v.z(), 1.0);
        let v2 = Vec3::new(1.0, 4.0, 2.0) / 2.0;
        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), 1.0);
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v += Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 8.0);
        assert_eq!(v.z(), 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 += 2.0;
        assert_eq!(v2.x(), 3.0);
        assert_eq!(v2.y(), 6.0);
        assert_eq!(v2.z(), 4.0);
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v -= Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), -2.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 -= 1.0;
        assert_eq!(v2.x(), 0.0);
        assert_eq!(v2.y(), 3.0);
        assert_eq!(v2.z(), 1.0);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v *= Vec3::new(1.0, 4.0, 2.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 16.0);
        assert_eq!(v.z(), 4.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 *= 2.0;
        assert_eq!(v2.x(), 2.0);
        assert_eq!(v2.y(), 8.0);
        assert_eq!(v2.z(), 4.0);
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(1.0, 4.0, 2.0);
        v /= Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.x(), 0.5);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 1.0);
        let mut v2 = Vec3::new(1.0, 4.0, 2.0);
        v2 /= 2.0;
        assert_eq!(v2.x(), 0.5);
        assert_eq!(v2.y(), 2.0);
        assert_eq!(v2.z(), 1.0);
    }
}