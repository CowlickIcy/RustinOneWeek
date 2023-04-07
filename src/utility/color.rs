#![allow(unused_variables)]
pub use super::vector::*;
pub use super::macros::*;

type Color = Vector3;

impl Color {
    pub fn write_color(pixel_color: &Color, samples_per_pixel: u32) {
        let mut r = pixel_color.x;
        let mut g = pixel_color.y;
        let mut b = pixel_color.z;

        if r != r {
            r = 0.0;
        }
        if g != g {
            g = 0.0;
        }
        if b != b {
            b = 0.0;
        }

        let scale = (1 / samples_per_pixel) as f64;
        r = f64::powf(scale * r, 1.0 / GAMMA);
        g = f64::powf(scale * g, 1.0 / GAMMA);
        b = f64::powf(scale * b, 1.0 / GAMMA);

    }
}
