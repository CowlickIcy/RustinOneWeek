pub use self::material::*;
pub use self::onb::*;
pub use self::pdf::*;
pub mod material;
pub mod onb;
pub mod pdf;
use crate::{geometry::HitRecord, texture::Texture, utility::*, geometry::Hittable};

pub trait Material: Sync {
    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn brdf(&self, _r_in: &Ray, _r_out: &Ray, rec: &HitRecord) -> Vector3 {
        Vector3::default()
    }
}
