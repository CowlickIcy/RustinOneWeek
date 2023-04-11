use self::material::*;

pub mod material;

use crate::utility::{*};

pub trait Material {
    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color;
    // fn scatter(r_in: &Ray, )
}
