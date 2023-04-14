use self::material::*;

pub mod material;

use crate::{geometry::HitRecord, texture::Texture, utility::*};

pub trait Material: Sync {
    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}
