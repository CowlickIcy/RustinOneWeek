use super::*;
use rand::Rng;

type Point3 = Vector3;

#[derive(Debug)]
pub struct Camera {
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
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
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
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u).normalize();

        let h = u * focus_dist * viewport_width;
        let v = v * focus_dist * viewport_width;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: lookfrom - h / 2.0 - v / 2.0 - w / focus_dist,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vector3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            rng.gen_range(self.time0..=self.time1),
        )
    }
}
