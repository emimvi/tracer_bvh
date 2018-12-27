pub mod vector;
pub mod ray;

pub use self::vector::Vector;
pub use self::ray::Ray;
pub use self::ray::RayT;

use std::f32;
use std::ops::{Mul, Add};

/// Enum representing on of the 3 spatial axes
#[derive(Copy, Clone, Debug)]
pub enum Axis { X, Y, Z }

/// Lerp between `a` and `b` at some distance `t` where t is in [0, 1]
/// and t = 0 returns `a` and t = 1 returns `b`
pub fn lerp<T: Mul<f32, Output = T> + Add<Output = T> + Copy>(t: f32, a: &T, b: &T) -> T {
    *a * (1.0 - t) + *b * t
}
