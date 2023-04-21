pub use self::material::*;
pub use self::onb::*;
pub use self::pdf::*;
pub mod material;
pub mod onb;
pub mod pdf;
use crate::geometry::hitrecord::ScatterRecord;
use crate::{geometry::HitRecord, geometry::Hittable, texture::Texture, utility::*};

pub trait Material: Sync {
    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn scatter_mc_methode(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn brdf(&self, _r_in: &Ray, _r_out: &Ray, _rec: &HitRecord) -> Vector3 {
        Vector3::default()
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scatterer: &Ray) -> f64 {
        0.0
    }
}
