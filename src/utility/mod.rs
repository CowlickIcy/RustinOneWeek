pub use self::aabb::*;
pub use self::color::*;
pub use self::macros::*;
pub use self::ray::*;
pub use self::vector::*;

pub mod aabb;
pub mod color;
pub mod macros;
pub mod ray;
pub mod vector;

pub type Point = Vector3;
pub type Color = Vector3;

impl Color {
    pub fn r(self) -> f64 {
        self.x()
    }

    pub fn g(self) -> f64 {
        self.y()
    }

    pub fn b(self) -> f64 {
        self.z()
    }
}
