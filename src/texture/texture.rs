use super::*;

pub struct CheckerTexture<T: Texture, S: Texture> {
    odd: T,
    even: S,
}

impl<T: Texture, S: Texture> CheckerTexture<T, S> {
    pub fn new(odd: T, even: S) -> CheckerTexture<T, S> {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture, S: Texture> Texture for CheckerTexture<T, S> {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sines < 0.0 {
            self.odd.get_color(u, v, p)
        } else {
            self.even.get_color(u, v, p)
        }
    }
}

pub struct NoiseTexture {}

pub struct ImageTexture {}
