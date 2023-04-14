pub use self::cube::*;
pub use self::hitrecord::*;
pub use self::hittablelist::*;
pub use self::rotate_y::*;
pub use self::translate::*;
pub use self::rectangular::*;

pub mod cube;
pub mod hitrecord;
pub mod hittablelist;
pub mod rotate_y;
pub mod translate;
pub mod rectangular;

use crate::material::*;
use crate::utility::*;

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}
