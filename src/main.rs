#![allow(dead_code)]
mod camera;
mod utility;
mod geometry;
mod texture;
mod material;
pub use utility::vector::*;

fn main() {
    let lookfrom = Vector3::new(1.0, 1.0, 1.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 0.0, 1.0);
    let vfov = 45.0;
    let aspect_ratio = 1.5;
    let aperture = 1.0;
    let focus_dist = 300.0;
    let t0 = 0.0;
    let t1 = 0.0;

    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        t0,
        t1,
    );

    println!("curr vec is:{:?}", cam);
}
