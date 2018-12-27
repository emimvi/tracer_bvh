#![allow(dead_code)]

pub mod bvh;
mod partition;
mod bbox;
mod linalg;

pub use bbox::BBox;
pub use linalg::vector;
pub use linalg::ray;
pub use self::bvh::BVH;

/// Trait implemented by scene objects that can report an AABB describing their bounds
pub trait Boundable {
    /// Get an AABB reporting the object's bounds over the time period
    /// The default implementation assumes the object isn't animated and
    /// simply returns its bounds. This is kind of a hack to use
    /// the BVH for animated geomtry (instances) and non-animated geometry (triangles).
    fn bounds(&self, start: f32, end: f32) -> BBox;


    ///// Have the object recompute its bounds for the time range. In the case
    ///// of deforming geometry this can rebuild acceleration structures for example.
    //fn update_deformation(&mut self, start: f32, end: f32) {}
}

#[cfg(test)]
mod tests {

use std::f32;
use linalg::{self, Vector, Ray};
pub use bbox::BBox;
pub use Boundable;
pub use self::bvh::BVH;

/// A sphere with user-specified radius located at the origin.
#[derive(Clone, Copy)]
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    /// Create a sphere with the desired radius
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius: radius }
    }
}

impl Boundable for Sphere {
    fn bounds(&self, _: f32, _: f32) -> BBox {
        BBox::span(Vector::new(-self.radius, -self.radius, -self.radius),
                   Vector::new(self.radius, self.radius, self.radius))
    }
}

    #[test]
    fn it_works() {
        let s = Sphere::new(3.);
        let v = vec!(s);

        BVH::unanimated(16, v);
    }
}
