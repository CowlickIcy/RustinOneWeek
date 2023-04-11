pub use self::vector::*;
pub use self::color::*;
pub use self::macros::*;
pub use self::ray::*;
pub use self::aabb::*;

pub mod vector;
pub mod color;
pub mod macros;
pub mod ray;
pub mod aabb;

pub type Point = Vector3;
pub type Color = Vector3;