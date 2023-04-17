#![allow(dead_code)]
mod camera;
mod geometry;
mod material;
mod texture;
mod utility;

use rand::Rng;
use rayon::prelude::*;
use std::io::{stderr, Write};

use camera::*;
use geometry::*;
use material::*;
use texture::*;
use utility::*;

fn ray_color(r: &Ray, background: Color, world: &Box<dyn Hittable>, depth: u64) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat.emitted(&rec);
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}

enum Scene {
    TwoSphere,
}

fn two_sphere() -> (Box<dyn Hittable>, Box<dyn Hittable>) {
    let mut world = HittableList::default();

    let top_mat = Lambertian::new(CheckerTexture::new(
        SolidTexture::new(Color::new(1.0, 1.0, 1.0)),
        SolidTexture::new(Color::new(0.3, 0.3, 1.0)),
    ));

    let bottom_mat = Lambertian::new(CheckerTexture::new(
        SolidTexture::new(Color::new(1.0, 1.0, 1.0)),
        SolidTexture::new(Color::new(0.3, 0.3, 1.0)),
    ));

    let top_sphere = Sphere::new(Point::new(0.0, 2.0, 0.0), 2.0, top_mat);
    let bottom_sphere = Sphere::new(Point::new(0.0, -2.0, 0.0), 2.0, bottom_mat);

    world.add(top_sphere);
    world.add(bottom_sphere);

    let mut lights = HittableList::default();
    (Box::new(world), Box::new(lights))
}
fn main() {
    // image settings
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u64 = 500;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 800;
    const MAX_DEPTH: u64 = 100;

    // scene
    let scene = Scene::TwoSphere;
    let (world, background, lights, camera) = match scene {
        Scene::TwoSphere => {
            let (world, lights) = two_sphere();
            let background = Color::new(0.7, 0.8, 1.0);
            let lookfrom = Point::new(10.0, 0.0, 0.0);
            let lookat = Point::new(0.0, 0.0, 0.0);
            let vup = Vector3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
            let vfov = 20.0;

            let camera = Camera::new(
                lookfrom,
                lookat,
                vup,
                vfov,
                ASPECT_RATIO,
                aperture,
                focus_dist,
                0.0,
                1.0,
            );
            (world, background, lights, camera)
        }
    };

    // ppm headera
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    // scan

    for h in (0..IMAGE_HEIGHT).rev() {
        eprint!("\r Scanlines remaining: {:3}", IMAGE_HEIGHT - 1 - h);
        stderr().flush().unwrap();
        for w in 0..IMAGE_WIDTH {
            let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_sample| {
                    let mut rng = rand::thread_rng();
                    let random_u = rng.gen::<f64>();
                    let random_v = rng.gen::<f64>();

                    let u = ((w as f64) + random_u) / (IMAGE_WIDTH - 1) as f64;
                    let v = ((h as f64) + random_v) / (IMAGE_HEIGHT - 1) as f64;

                    let r = camera.get_ray(u, v);

                    ray_color(&r, background, &world, MAX_DEPTH)
                })
                .sum();
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
        eprintln!("Done.");
    }
}
