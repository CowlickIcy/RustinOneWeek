use std::cmp::min;

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

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
    resolution: usize,
}

impl NoiseTexture {
    pub fn new(scale: f64, resolution: usize) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(resolution),
            scale: scale,
            resolution: resolution,
        }
    }
}

impl Texture for NoiseTexture {
    fn get_color(&self, _u: f64, _v: f64, p: &Point) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(p, self.scale))
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> ImageTexture {
        ImageTexture {
            data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn get_color(&self, u: f64, v: f64, _p: &Point) -> Color {
        let mut i = (u.clamp(0.0, 1.0) * self.width as f64) as usize;
        let mut j = ((1.0 - v).clamp(0.0, 1.0) * self.height as f64) as usize;

        i = min(i, self.width - 1);
        j = min(j, self.height - 1);

        let idx = 3 * i + 3 * self.width * j;
        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}
