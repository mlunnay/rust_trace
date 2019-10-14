#[cfg(any(not(feature = "simd"), not(target_feature = "avx2")))]
pub mod vec;
#[cfg(any(not(feature = "simd"), not(target_feature = "avx2")))]
pub use vec::*;
#[cfg(all(feature = "simd", target_feature = "avx2"))]
pub mod vec3_simd;
#[cfg(all(feature = "simd", target_feature = "avx2"))]
pub use vec3_simd::*;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod util;
pub mod material;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod renderer;
pub mod cuboid;
pub mod modify;
pub mod quaternion;
pub mod constant_medium;

pub const EPSILON:f64 = 0.0001;