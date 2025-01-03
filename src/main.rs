#![allow(dead_code, unused_variables)]
mod camera;
mod geometry;
mod material;
mod texture;
mod utility;

use image::*;
use rand::Rng;
use rayon::prelude::*;
use std::io::{stderr, Write};

use camera::*;
use geometry::*;
use material::*;
use texture::*;
use utility::*;

fn ray_color(
    r: &Ray,
    background: Color,
    world: &Box<dyn Hittable>,
    lights: &Box<dyn Hittable>,
    depth: u64,
) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat.emitted(&rec);

        // let use_old_methom = true;

        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, lights, depth - 1)
        // if let Some(srec) = rec.mat.scatter_mc_methode(r, &rec) {
        //     match srec {
        //         ScatterRecord::Specular {
        //             specular_ray,
        //             attenuation,
        //         } => {
        //             return attenuation
        //                 * ray_color(&specular_ray, background, world, lights, depth - 1);
        //         }
        //         ScatterRecord::Scatter { pdf, attenuation } => {
        //             let hittable_pdf = PDF::hittable_pdf(rec.p, lights);
        //             let mixture_pdf = PDF::mixture_pdf(&hittable_pdf, &pdf);
        //             let scattered = Ray::new(rec.p, mixture_pdf.generate(), r.time());
        //             let pdf_value = mixture_pdf.value(scattered.dir());
        //             return emitted
        //                 + attenuation
        //                     * rec.mat.scattering_pdf(r, &rec, &scattered)
        //                     * ray_color(&scattered, background, world, lights, depth - 1)
        //                     / pdf_value;
        //         }
        //         ScatterRecord::Microfacet { pdf } => {
        //             let hittable_pdf = PDF::hittable_pdf(rec.p, lights);
        //             let mixture_pdf = PDF::mixture_pdf(&hittable_pdf, &pdf);
        //             let scattered = Ray::new(rec.p, mixture_pdf.generate(), r.time());
        //             let pdf_value = mixture_pdf.value(scattered.dir());
        //             return emitted
        //                 + rec.mat.brdf(r, &scattered, &rec)
        //                     * ray_color(&scattered, background, world, lights, depth - 1)
        //                     / pdf_value;
        //         }
        // }
        //
        } else {
            emitted
        }
    } else {
        background
    }
}

enum Scene {
    TwoSphere,
    CornellBox,
    CornelBoxTest,
}

fn two_sphere() -> (Box<dyn Hittable>, Box<dyn Hittable>) {
    let mut world = HittableList::default();
    let lights = HittableList::default();

    let top_mat = Lambertian::new(CheckerTexture::new(
        SolidTexture::new(Color::new(1.0, 1.0, 1.0)),
        SolidTexture::new(Color::new(0.7, 0.3, 0.5)),
    ));

    let bottom_mat = Lambertian::new(SolidTexture::new(Color::new(0.4, 0.31, 1.0)));

    let top_sphere = Sphere::new(Point::new(0.0, 2.0, 0.0), 2.0, top_mat);
    let bottom_sphere = Sphere::new(Point::new(0.0, -2.0, 0.0), 2.0, bottom_mat);

    world.add(top_sphere);
    world.add(bottom_sphere);

    (Box::new(world), Box::new(lights))
}

fn cornell_box_test() -> (Box<dyn Hittable>, Box<dyn Hittable>) {
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let m_white = Lambertian::new(SolidTexture::new(Color::new(0.73, 0.73, 0.73)));
    let m_red = Lambertian::new(SolidTexture::new(Color::new(0.65, 0.05, 0.05)));
    let m_green = Lambertian::new(SolidTexture::new(Color::new(0.12, 0.45, 0.15)));
    let m_light = DiffuseLight::new(SolidTexture::new(Color::new(15.0, 15.0, 15.0)));
    let rect_light = AARect::new(Plane::XZ, 213.0, 343.0, 227.0, 332.0, 554.0, m_light);

    world.add(Sphere::new(Point::new(0.0, 0.0, 0.0), 200.0, m_white));
    world.add(Sphere::new(Point::new(200.0, 0.0, 0.0), 200.0, m_red));

    lights.add(rect_light);
    (Box::new(world), Box::new(lights))
}

fn cornell_box() -> (Box<dyn Hittable>, Box<dyn Hittable>) {
    let mut world = HittableList::default();
    let mut lights = HittableList::default();

    let m_red = Lambertian::new(SolidTexture::new(Color::new(0.65, 0.05, 0.05)));
    let m_white = Lambertian::new(SolidTexture::new(Color::new(0.73, 0.73, 0.73)));
    let m_green = Lambertian::new(SolidTexture::new(Color::new(0.12, 0.45, 0.15)));
    let m_blue = Lambertian::new(SolidTexture::new(Color::new(0.051, 0.459, 1.000)));
    let m_lemon_yellow = Lambertian::new(SolidTexture::new(Color::new(0.894, 0.941, 0.141)));
    let m_cotinga_purple = Lambertian::new(SolidTexture::new(Color::new(0.204, 0.000, 0.349)));
    let m_dielectric = Dielectric::new(1.5);
    let m_metal = Metallic::new(Color::new(0.8, 0.85, 0.88), 0.0);
    let m_light = DiffuseLight::new(SolidTexture::new(Color::new(15.0, 15.0, 15.0)));
    let m_pbr = PBR::new(
        SolidTexture::new(Color::new(0.6, 0.7, 0.2)),
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    );
    let rect_light = AARect::new(
        Plane::XZ,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        m_light.clone(),
    );

    world.add(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        m_green,
    ));
    world.add(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, m_red));
    world.add(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        m_white.clone(),
    ));
    world.add(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        m_blue,
    ));
    world.add(rect_light.clone());
    world.add(AARect::new(
        Plane::XZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        m_lemon_yellow,
    ));

    world.add(Translate::new(
        Rotate::new(
            RotateAxis::Y,
            Cube::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(165.0, 165.0, 165.0),
                m_white,
            ),
            -18.0,
        ),
        Vector3::new(130.0, 0.0, 65.0),
    ));

    world.add(Translate::new(
        Rotate::new(
            RotateAxis::Y,
            Cube::new(
                Point::new(0.0, 0.0, 0.0),
                Point::new(165.0, 330.0, 165.0),
                m_metal,
            ),
            15.0,
        ),
        Vector3::new(265.0, 0.0, 295.0),
    ));

    lights.add(rect_light);

    (Box::new(world), Box::new(lights))
}
fn main() {
    // image settings
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 500;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u64 = 1000;
    const MAX_DEPTH: u64 = 100;

    // scene
    let scene = Scene::CornellBox;
    let (world, background, lights, camera) = match scene {
        Scene::TwoSphere => {
            let (world, lights) = two_sphere();
            let background = Color::new(0.7, 0.8, 1.0);
            let lookfrom = Point::new(13.0, 2.0, 3.0);
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
        Scene::CornellBox => {
            let (world, lights) = cornell_box();
            let backgournd = Color::new(0.0, 0.0, 0.0);

            let lookfrom = Point::new(278.0, 278.0, -800.0);
            let lookat = Point::new(278.0, 278.0, 0.0);
            let vup = Vector3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera = Camera::new(
                lookfrom,
                lookat,
                vup,
                40.0,
                ASPECT_RATIO,
                aperture,
                dist_to_focus,
                0.0,
                1.0,
            );

            (world, backgournd, lights, camera)
        }
        Scene::CornelBoxTest => {
            let (world, lights) = cornell_box_test();
            let backgournd = Color::new(0.5, 0.5, 0.5);

            let lookfrom = Point::new(278.0, 278.0, -800.0);
            let lookat = Point::new(278.0, 278.0, 0.0);
            let vup = Vector3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera = Camera::new(
                lookfrom,
                lookat,
                vup,
                20.0,
                ASPECT_RATIO,
                aperture,
                dist_to_focus,
                0.0,
                1.0,
            );

            (world, backgournd, lights, camera)
        }
    };

    // create a new ImgBuf with
    let mut imgbuf: RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_WIDTH);

    // image scan
    for y in 0..IMAGE_HEIGHT {
        eprint!("\r Scanlines remaining: {:3}", y);
        stderr().flush().unwrap();
        for x in 0..IMAGE_WIDTH {
            let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_sample| {
                    let mut rng = rand::thread_rng();
                    let random_u = rng.gen::<f64>();
                    let random_v = rng.gen::<f64>();

                    let u = ((x as f64) + random_u) / (IMAGE_WIDTH - 1) as f64;
                    let v = ((y as f64) + random_v) / (IMAGE_HEIGHT - 1) as f64;

                    let r = camera.get_ray(u, v);

                    ray_color(&r, background, &world, &lights, MAX_DEPTH)
                })
                .sum();
            let pixel = imgbuf.get_pixel_mut(x, IMAGE_HEIGHT - 1 - y);

            *pixel = image::Rgb(pixel_color.format_channel(SAMPLES_PER_PIXEL));
        }
        eprintln!("Done.");
    }

    imgbuf.save("image.png").unwrap();
}
