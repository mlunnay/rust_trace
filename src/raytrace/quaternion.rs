use super::vec::Vec3;
use super::util::approx_equal;

/// A quaternion rotation
#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    /// the scalar part of the quaternion
    s: f64,
    /// the vector part of the quaternion
    v: Vec3
}

impl Quaternion {
    /// Create a quaternion from a scalar and three imaginary components.
    pub fn new(s: f64, xi: f64, yk: f64, zi: f64) -> Quaternion {
        Quaternion{s, v: Vec3{x: xi, y: yk, z: zi}}
    }

    /// Create a quaternion from a scalar and a vector.
    pub fn from_sv(s: f64, v: Vec3) -> Quaternion {
        Quaternion{s, v}
    }

    /// Creates an empty quaternion
    pub fn zero() -> Quaternion {
        Quaternion{s: 0.0, v: Vec3{x: 0.0, y: 0.0, z: 0.0}}
    }

    /// Creates an identity quaternion
    pub fn identity() -> Quaternion {
        Quaternion{s: 1.0, v: Vec3{x: 0.0, y: 0.0, z: 0.0}}
    }

    pub fn elements(self) -> [f64; 4] {
        [self.s, self.v.x, self.v.y, self.v.z]
    }

    // Returns the dot product of this quaternion with another.
    pub fn dot(self, other: Quaternion) -> f64 {
        self.s * other.s + Vec3::dot(&self.v, &other.v)
    }

    /// Returns the conjugate (inverse) of the quaternion.
    pub fn conjugate(self) -> Quaternion {
        Quaternion{s: self.s, v: -self.v}
    }

    /// Returns the length of the quaternion.
    pub fn length(self) -> f64 {
        (self.s * self.s + self.v.length_squared()).sqrt()
    }

    /// Computes the quared length of this quaternion
    pub fn length_squared(self) -> f64 {
        self.s * self.s + self.v.length_squared()
    }

    /// Normalize the quaternion to unit length
    pub fn normalize(&mut self) -> Quaternion { 
        let length = 1.0 / self.length();
        Quaternion{s: self.s * length, v: Vec3{x: self.v.x * length, y: self.v.y * length, z: self.v.z * length}}
    }

    /// Returns the Eular angles for this quaternion
    pub fn to_eular_angles(self) -> Vec3 {
        // taken from cgmath crate
        let sig = 0.4999999;

        let (qw, qx, qy, qz) = (self.s, self.v.x, self.v.y, self.v.z);
        let (sqw, sqx, sqy, sqz) = (qw * qw, qx * qx, qy * qy, qz * qz);

        let unit = sqx + sqz + sqy + sqw;
        let test = qx * qz + qy * qw;

        // We set x to zero and z to the value, but the other way would work too.
        if test > sig * unit {
            // x + z = 2 * atan(x / w)
            Vec3 {
                x: 0.0,
                y: std::f64::consts::FRAC_PI_2,
                z: f64::atan2(qx, qw) * 2.0,
            }
        } else if test < -sig * unit {
            // x - z = 2 * atan(x / w)
            Vec3 {
                x: 0.0,
                y: std::f64::consts::FRAC_PI_2,
                z: -f64::atan2(qx, qw) * 2.0,
            }
        } else {
            // Using the quat-to-matrix equation from either
            // http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToMatrix/index.htm
            // or equation 15 on page 7 of
            // http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf
            // to fill in the equations on page A-2 of the NASA document gives the below.
            Vec3 {
                x: f64::atan2(2.0 * (-qy * qz + qx * qw), 1.0 - 2.0 * (sqx + sqy)),
                y: f64::asin(2.0 * (qx * qz + qy * qw)),
                z: f64::atan2(2.0 * (-qx * qy + qz * qw), 1.0 - 2.0 * (sqy + sqz)),
            }
        }
    }

    /// Creates a Quaternion from a rotation around an axis.
    pub fn from_rotation_axis(axis: Vec3, angle: f64) -> Quaternion {
        let sin = (angle / 2.0).sin();
        let axis = axis.normalize();
        let s = (angle / 2.0).cos();
        let x = axis.x * sin;
        let y = axis.y * sin;
        let z = axis.z * sin;
        Quaternion{s: s, v: Vec3{x, y ,z}}
    }

    /// Create a Quaternion from a Vec3 of eular rotations
    pub fn from_eular_angles(v: Vec3) -> Quaternion {
        let s_x = (v.x * 0.5).sin();
        let s_y = (v.y * 0.5).sin();
        let s_z = (v.z * 0.5).sin();
        let c_x = (v.x * 0.5).cos();
        let c_y = (v.y * 0.5).cos();
        let c_z = (v.z * 0.5).cos();

        Quaternion::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }

    /// Create a Quaternion from yaw pitch and roll angles.
    pub fn from_yaw_pitch_roll(yaw: f64, pitch: f64, roll:f64) -> Quaternion {
        Self::from_eular_angles(Vec3::new(pitch, yaw, roll))
    }

    /// Calculates the rotation Quaternion for the angle betweel to vectors.
    pub fn from_arc(src: Vec3, dst: Vec3, fallback: Option<Vec3>) -> Quaternion {
        let length_avg = (src.length_squared() * dst.length_squared()).sqrt();
        let dot = Vec3::dot(&src, &dst);
        if approx_equal(dot, length_avg) {
            Quaternion::identity()
        }
        else if approx_equal(dot, -length_avg) {
            // use the fallbak axis or generate a new axis.
            let axis = fallback.unwrap_or_else(|| {
                let mut v = Vec3::cross(&Vec3::unit_x(), &src);
                if v.is_zero_length() {
                    v = Vec3::cross(&Vec3::unit_y(), &src);
                }
                v.normalize()
            });
            Quaternion::from_rotation_axis(axis, std::f64::consts::PI)
        }
        else {
            Quaternion::from_sv(length_avg + dot, Vec3::cross(&src, &dst).normalize())
        }
    }

    /// Do a normalized lerp between this Quarternion and another.
    pub fn nlerp(self, other: Quaternion, ammount: f64) -> Quaternion {
        (self * (1.0 - ammount) + other * ammount).normalize()
    }

    /// Does a spherical linear interplation. Expects quaternions to be normalize first.
    pub fn slerp(self, other:  Quaternion, ammount: f64) -> Quaternion {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        let threshold = 0.9995f64;
        let mut dot = Quaternion::dot(self, other);

        if dot > threshold {
            // too close just use nlerp
            self.nlerp(other, ammount)
        }
        else {
            dot = if dot > 1.0 { 1.0 } else if dot < -1.0 { -1.0 } else { dot };
            let theta = dot.acos();
            let scale1 = f64::sin(theta * (1.0 - ammount));
            let scale2 = f64::sin(theta * ammount);

            (self * scale1 + other * scale2) * f64::sin(theta)
        }
    }

    pub fn rotate_vector(self, other: Vec3) -> Vec3 {
        self * other
    }
}

// TODO: implement matrix conversion

impl From<[f64;4]> for Quaternion {
    fn from(arr: [f64;4]) -> Quaternion { Quaternion{s: arr[0], v: Vec3{x: arr[1], y: arr[2], z: arr[3]}} }
}

impl From<(f64, f64, f64, f64)> for Quaternion {
    fn from(tup: (f64, f64, f64, f64)) -> Quaternion { Quaternion{s: tup.0, v: Vec3{x: tup.1, y: tup.2, z: tup.3}} }
}

impl core::ops::Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Quaternion {
        Quaternion{s: -self.s, v: -self.v}
    }
}

impl core::ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s + rhs.s, self.v + rhs.v)
    }
}

impl core::ops::AddAssign<Quaternion> for Quaternion {
    fn  add_assign(&mut self, rhs: Quaternion) {
        *self = Quaternion{s: self.s + rhs.s, v: self.v + rhs.v}
    }
}

impl core::ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s - rhs.s, self.v - rhs.v)
    }
}

impl core::ops::SubAssign<Quaternion> for Quaternion {
    fn  sub_assign(&mut self, rhs: Quaternion) {
        *self = Quaternion{s: self.s - rhs.s, v: self.v - rhs.v}
    }
}

impl core::ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s * rhs.s, self.v * rhs.v)
    }
}

impl core::ops::MulAssign<Quaternion> for Quaternion {
    fn  mul_assign(&mut self, rhs: Quaternion) {
        *self = Quaternion{s: self.s * rhs.s, v: self.v * rhs.v}
    }
}

impl core::ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f64) -> Quaternion {
        Quaternion::from_sv(self.s * rhs, self.v * rhs)
    }
}

impl core::ops::MulAssign<f64> for Quaternion {
    fn  mul_assign(&mut self, rhs: f64) {
        *self = Quaternion{s: self.s * rhs, v: self.v * rhs}
    }
}

impl core::ops::Div<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: Quaternion) -> Quaternion {
        Quaternion::from_sv(self.s / rhs.s, self.v / rhs.v)
    }
}

impl core::ops::DivAssign<Quaternion> for Quaternion {
    fn  div_assign(&mut self, rhs: Quaternion) {
        *self = Quaternion{s: self.s / rhs.s, v: self.v / rhs.v}
    }
}

impl core::ops::Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f64) -> Quaternion {
        Quaternion::from_sv(self.s / rhs, self.v / rhs)
    }
}

impl core::ops::DivAssign<f64> for Quaternion {
    fn  div_assign(&mut self, rhs: f64) {
        *self = Quaternion{s: self.s / rhs, v: self.v / rhs}
    }
}

impl core::ops::Mul<Vec3> for Quaternion {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let tmp = Vec3::cross(&self.v, &rhs) + rhs * self.s;
        Vec3::cross(&self.v, &tmp) * 2.0 + rhs
    }
}

impl core::ops::Mul<Quaternion> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Quaternion) -> Vec3 {
       rhs * self
    }
}