pub use self::cube::*;
pub use self::hitrecord::*;
pub use self::scatterrecord::*;
pub use self::hittablelist::*;
pub use self::rotate::*;
pub use self::translate::*;
pub use self::rotate::*;
pub use self::rectangular::*;
pub use self::sphere::*;
pub use self::constantmedium::*;
pub use self::mesh::*;
pub use self::bvh::*;

pub mod cube;
pub mod sphere;
pub mod hitrecord;
pub mod hittablelist;
pub mod translate;
pub mod rotate;
pub mod rectangular;
pub mod constantmedium;
pub mod mesh;
pub mod bvh;
pub mod scatterrecord;

use crate::material::*;
use crate::utility::*;

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
