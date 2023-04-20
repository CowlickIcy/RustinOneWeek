pub use self::bvh::*;
pub use self::constantmedium::*;
pub use self::cube::*;
pub use self::hitrecord::*;
pub use self::hittablelist::*;
pub use self::mesh::*;
pub use self::rectangular::*;
pub use self::rotate::*;
pub use self::rotate::*;
pub use self::sphere::*;
pub use self::translate::*;

pub mod bvh;
pub mod constantmedium;
pub mod cube;
pub mod hitrecord;
pub mod hittablelist;
pub mod mesh;
pub mod rectangular;
pub mod rotate;
pub mod sphere;
pub mod translate;

use crate::material::*;
use crate::utility::*;

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _o: Point, _v: Vector3) -> f64 {
        0.0
    }
}
