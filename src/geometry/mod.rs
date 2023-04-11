pub use self::cube::*;
pub use self::hitrecord::*;
pub use self::translate::*;
pub use self::rotate_y::*;
pub use self::hittablelist::*;

pub mod cube;
pub mod hitrecord;
pub mod translate;
pub mod rotate_y;
pub mod hittablelist;

use crate::material::*;
use crate::utility::{*};

pub struct Geometry;

pub trait Hittable{
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0:f64, time1:f64, output_box: &mut Aabb) -> bool;
}

