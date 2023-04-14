use self::material::*;

pub mod material;

use crate::{utility::*, geometry::HitRecord};

pub trait Material: Sync {
    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: Color, scattered: Color) -> bool;

}
