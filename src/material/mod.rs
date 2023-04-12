use self::material::*;

pub mod material;

use crate::utility::*;

#[derive(Clone)]
pub struct BaseMaterial {
    id: u32,
}
impl BaseMaterial {
    pub fn new(id: u32) -> BaseMaterial {
        BaseMaterial { id }
    }
}
pub trait Material {
    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color;
    // fn scatter(r_in: &Ray, )
}

impl Material for BaseMaterial {
    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
