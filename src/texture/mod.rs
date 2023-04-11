#!(allow(dead_code, unused_variables))
pub use self::texture::*;

pub mod texture;

use crate::utility::macros::*;
use crate::utility::vector::*;

type Point = Vector3;
type Color = Vector3;


pub struct Texture {
    color: Color,
}
pub trait TextureColor {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color;
}

impl TextureColor for Texture {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color {
        self.color
    }
}
