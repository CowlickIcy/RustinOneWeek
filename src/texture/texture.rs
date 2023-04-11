use super::*;

pub struct SolidColor {
    pub color_val: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color_val: color }
    }

    pub fn new_with_color(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_val: Color { x: r, y: g, z: b },
        }
    }
}

impl TextureColor for SolidColor {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color {
        self.color_val
    }
}

pub struct CheckerTexture {
    pub odd: Texture,
    pub even: Texture,
}

impl CheckerTexture {
    pub fn new(odd: Texture, even: Texture) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
    pub fn new_with_color(odd: Color, even: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Texture { color: odd },
            even: Texture { color: even },
        }
    }
}

impl TextureColor for CheckerTexture {
    fn get_color(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.get_color(u, v, p)
        } else {
            self.even.get_color(u, v, p)
        }
    }
}

pub struct NoiseTexture {}

pub struct ImageTexture {}
