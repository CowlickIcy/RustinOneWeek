#!(allow(dead_code, unused_variables))
pub use self::texture::*;
pub use self::perlin::*;

pub mod texture;
pub mod perlin;

use crate::utility::{macros::*, *};
pub trait Texture {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color;
}
