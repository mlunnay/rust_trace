//! Provides instance classes to modify a given hittable

use super::aabb::AABB;
use super::vec::Vec3;
use super::ray::Ray;
use super::quaternion::Quaternion;
use super::hittable::{HitRecord, Hittable};
use std::rc::Rc;

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Translate{object, offset}
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray = Ray::new(r.origin - self.offset, r.direction);
        match self.object.hit(ray, t_min, t_max) {
            Some(rec) => Some(HitRecord{
                p: rec.p + self.offset,
                ..rec
            }),
            None => None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        match self.object.bounding_box() {
            Some(bbox) => Some(AABB::new(bbox.min + self.offset, bbox.max + self.offset)),
            None => None
        }
    }
}

pub trait Rotation: Hittable {
    fn rotation(&self) -> Quaternion;
    fn object(&self) -> Rc<dyn Hittable>;
    fn bbox(&self) -> Option<AABB>;

    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let q = Self::rotation(self);
        let qr = q.conjugate();
        let rotated_ray = Ray::new(qr * r.origin, qr * r.direction);
        match Self::object(self).hit(rotated_ray, t_min, t_max) {
            Some(rec) => {
                let p = q * rec.p;
                let normal = q * rec.normal;
                Some(HitRecord{
                    p,
                    normal,
                    ..rec
                })
            }
            None => None
        }

    }

    fn bounding_box(&self) -> Option<AABB> {
        Self::bbox(self)
    }

    fn rotate_bounding_box(q: Quaternion, bounding_box: AABB) -> AABB {
        let mut vectors: [Vec3; 8] = [Vec3::zero(); 8];
        vectors[0] = q * Vec3{x: bounding_box.min.x, y: bounding_box.min.y, z: bounding_box.min.z};
        vectors[1] = q * Vec3{x: bounding_box.min.x, y: bounding_box.min.y, z: bounding_box.max.z};
        vectors[2] = q * Vec3{x: bounding_box.min.x, y: bounding_box.max.y, z: bounding_box.min.z};
        vectors[3] = q * Vec3{x: bounding_box.min.x, y: bounding_box.max.y, z: bounding_box.max.z};
        vectors[4] = q * Vec3{x: bounding_box.max.x, y: bounding_box.min.y, z: bounding_box.min.z};
        vectors[5] = q * Vec3{x: bounding_box.max.x, y: bounding_box.min.y, z: bounding_box.max.z};
        vectors[6] = q * Vec3{x: bounding_box.max.x, y: bounding_box.max.y, z: bounding_box.min.z};
        vectors[7] = q * Vec3{x: bounding_box.max.x, y: bounding_box.max.y, z: bounding_box.max.z};

        let mut min = Vec3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
        let mut max = Vec3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN);

        for i in 0..8 {
            let v = vectors[i];
            if v.x < min.x {
                min.x = v.x;
            }
            if v.x > max.x {
                max.x = v.x
            }
            if v.y < min.y {
                min.y = v.y;
            }
            if v.y > max.y {
                max.y = v.y
            }
            if v.z < min.z {
                min.z = v.z;
            }
            if v.z > max.z {
                max.z = v.z
            }
        }

        AABB::new(min, max)
    }
}

macro_rules! impl_rotation {
    ($op: ident) => {
        impl Rotation for $op {
            fn rotation(&self) -> Quaternion {
                self._rotation
            }

            fn object(&self) -> Rc<dyn Hittable> {
                self._object.clone()
            }

            fn bbox(&self) -> Option<AABB> {
                self._bounding_box
            }
        }
    };
}

macro_rules! impl_hittable {
    ($op: ident) => {
        impl Hittable for $op {
            #[inline]
            fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                Rotation::hit(self, r, t_min, t_max)
            }

            #[inline]
            fn bounding_box(&self) -> Option<AABB> {
                Rotation::bounding_box(self)
            }
        }
    };
}

macro_rules! impl_axis_rotation {
    ($op: ident, $axis: expr) => {
        impl $op {
            pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
                let q = Quaternion::from_rotation_axis($axis, angle);
                let boundingbox: Option<AABB> =  match object.bounding_box() {
                    Some(bbox) => Some(Self::rotate_bounding_box(q, bbox)),
                    None => None
                };
                RotateY{_object: object, _rotation: q, _bounding_box: boundingbox}
            }

            pub fn set_angle(&mut self, angle: f64) {
                self._rotation = Quaternion::from_rotation_axis($axis, angle);
                if let Some(bbox) = self._bounding_box {
                    self._bounding_box = Some(Self::rotate_bounding_box(self._rotation, bbox));
                };
            }
        }
    };
}

pub struct Rotate {
    _object: Rc<dyn Hittable>,
    _rotation: Quaternion,
    _bounding_box: Option<AABB>
}

impl Rotate {
    pub fn new(object: Rc<dyn Hittable>, rotation: Quaternion) -> Rotate {
        let boundingbox: Option<AABB> =  match object.bounding_box() {
            Some(bbox) => Some(Self::rotate_bounding_box(rotation, bbox)),
            None => None
        };
        Rotate{_object: object, _rotation: rotation, _bounding_box: boundingbox}
    }

    pub fn set_rotation(&mut self, q: Quaternion) {
        self._rotation = q;
        if let Some(bbox) = self._bounding_box {
            self._bounding_box = Some(Self::rotate_bounding_box(q, bbox));
        };
    }

    pub fn set_rotation_from_axis_angle(&mut self, axis: Vec3, angle: f64) {
        self._rotation = Quaternion::from_rotation_axis(axis, angle);
        if let Some(bbox) = self._bounding_box {
            self._bounding_box = Some(Self::rotate_bounding_box(self._rotation, bbox));
        };
    }
}

impl_hittable!(Rotate);
impl_rotation!(Rotate);

pub struct RotateX {
    _rotation: Quaternion,
    _object: Rc<dyn Hittable>,
    _bounding_box: Option<AABB>
}
impl_axis_rotation!(RotateX, Vec3::unit_x());
impl_rotation!(RotateX);
impl_hittable!(RotateX);

pub struct RotateY {
    _rotation: Quaternion,
    _object: Rc<dyn Hittable>,
    _bounding_box: Option<AABB>
}
impl_axis_rotation!(RotateY, Vec3::unit_y());
impl_rotation!(RotateY);
impl_hittable!(RotateY);

pub struct RotateZ {
    _rotation: Quaternion,
    _object: Rc<dyn Hittable>,
    _bounding_box: Option<AABB>
}
impl_axis_rotation!(RotateZ, Vec3::unit_z());
impl_rotation!(RotateZ);
impl_hittable!(RotateZ);