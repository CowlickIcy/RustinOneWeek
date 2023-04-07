use utility::macros::*;
use utility::ray::*;
use utility::vector::*;

type Point3 = Vector3;

struct Camera {
    // lookfrom: Point3,
    // lookat: Point3,
    // vup: Vector3,
    // vfov: f64,
    // aspect_ratio: f64,
    // aperture: f64,
    // focus_dist: f64,
    // _time0: f64,
    // _time1: f64,
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vector3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Camera {
        let theta = degress_to_radians(vfov);
        let h = f64::tan(theta / 2);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        w = Vector3::unit_vector(lookfrom - lookat);
        u = 
    }
}
