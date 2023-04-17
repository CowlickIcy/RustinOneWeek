pub use self::aabb::*;
pub use self::color::*;
pub use self::macros::*;
pub use self::ray::*;
pub use self::vector::*;

pub mod aabb;
pub mod color;
pub mod macros;
pub mod ray;
pub mod vector;

pub type Point = Vector3;
pub type Color = Vector3;

impl Color {
    pub fn r(self) -> f64 {
        self.x()
    }

    pub fn g(self) -> f64 {
        self.y()
    }

    pub fn b(self) -> f64 {
        self.z()
    }

    pub fn format_color(self, sampler_per_pixel: u64) -> String {
        let ir = (256.0
            * (self[0] / (sampler_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ig = (256.0
            * (self[1] / (sampler_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        let ib = (256.0
            * (self[2] / (sampler_per_pixel as f64))
                .sqrt()
                .clamp(0.0, 0.999)) as u64;
        format!("{} {} {}", ir, ig, ib)
    }
}
