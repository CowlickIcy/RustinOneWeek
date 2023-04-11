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
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vector3::unit_vector(&lookfrom.sub(lookat));
        let u = Vector3::unit_vector(&vup.cross(&w));
        let v = Vector3::unit_vector(&w.cross(&u));

        let h = u.mul_with_num(focus_dist * viewport_width);
        let v = v.mul_with_num(focus_dist * viewport_width);

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: lookfrom
                - h.div_with_num(2.0)
                - v.div_with_num(2.0)
                - w.mul_with_num(focus_dist),
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vector3::random_in_unit_disk().mul_with_num(self.lens_radius);
        let offset = self.u.mul_with_num(rd.x) + self.v.mul_with_num(rd.y);
        let mut rng = rand::thread_rng();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + self.horizontal.mul_with_num(s)
                + self.vertical.mul_with_num(t)
                - self.origin
                - offset,
            rng.gen_range(self.time0..=self.time1),
        )
    }
}
