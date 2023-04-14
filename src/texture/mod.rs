#!(allow(dead_code, unused_variables))
pub use self::texture::*;
pub use self::perlin::*;

pub mod texture;
pub mod perlin;

use crate::utility::{macros::*, *};
pub trait Texture {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct BaseTexture {
    color: Color,
}

impl BaseTexture {
    pub fn new(color: Color) -> BaseTexture {
        BaseTexture { color }
    }
}

impl Texture for BaseTexture {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color {
        self.color
    }
}
