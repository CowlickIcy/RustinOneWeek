use super::*;

#[derive(Debug, Clone)]
pub enum RotateAxis {
    X,
    Y,
    Z,
}

impl RotateAxis {
    pub fn get_axis_index(axis: &RotateAxis) -> (usize, usize, usize) {
        match axis {
            RotateAxis::X => (0, 1, 2),
            RotateAxis::Y => (1, 0, 2),
            RotateAxis::Z => (2, 0, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rotate<H: Hittable> {
    axis: RotateAxis,
    sin_theta: f64,
    cos_theta: f64,
    hittable: H,
    rbox: Option<Aabb>,
}

impl<H: Hittable> Rotate<H> {
    pub fn new(axis: RotateAxis, hittable: H, angle: f64) -> Rotate<H> {
        let (r_axis, a_axis, b_axis) = RotateAxis::get_axis_index(&axis);
        let radiants = degress_to_radians(angle);
        let sin_theta = radiants.sin();
        let cos_theta = radiants.cos();

        let rbox = hittable.bounding_box(0.0, 1.0).map(|mut rbox| {
            let mut min = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = Vector3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

            for iter in 0..=1 {
                for jter in 0..=1 {
                    for kter in 0..=1 {
                        let r = kter as f64 * rbox.max.x() + (1 - kter) as f64 * rbox.min.x();
                        let a = iter as f64 * rbox.max.y() + (1 - iter) as f64 * rbox.min.y();
                        let b = jter as f64 * rbox.max.z() + (1 - jter) as f64 * rbox.min.z();

                        let new_a = cos_theta * a + sin_theta * b;
                        let new_b = -sin_theta * a + cos_theta * b;

                        min[a_axis] = min[a_axis].min(new_a);
                        min[b_axis] = min[b_axis].min(new_b);
                        min[r_axis] = min[r_axis].min(r);

                        max[a_axis] = max[a_axis].max(new_a);
                        max[b_axis] = max[b_axis].max(new_b);
                        max[r_axis] = max[r_axis].max(r);
                    }
                }
            }
            rbox.min = min;
            rbox.max = max;
            rbox
        });

        Rotate {
            axis,
            sin_theta,
            cos_theta,
            hittable,
            rbox,
        }
    }
}
impl<H: Hittable> Hittable for Rotate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = RotateAxis::get_axis_index(&self.axis);
        let mut origin = r.origin();
        let mut direction = r.dir();

        origin[a_axis] =
            &self.cos_theta * r.origin()[a_axis] - &self.sin_theta * r.origin()[b_axis];
        origin[b_axis] =
            &self.sin_theta * r.origin()[a_axis] + &self.cos_theta * r.origin()[b_axis];

        direction[a_axis] = &self.cos_theta * r.dir()[a_axis] - &self.sin_theta * r.dir()[b_axis];
        direction[b_axis] = &self.sin_theta * r.dir()[a_axis] + &self.cos_theta * r.dir()[b_axis];

        let rotate_ray = Ray::new(origin, direction, r.time());

        self.hittable.hit(&rotate_ray, t_min, t_max).map(|mut hit| {
            let mut position = hit.p;
            let mut normal = hit.normal;
            position[a_axis] = &self.cos_theta * hit.p[a_axis] - &self.sin_theta * hit.p[b_axis];
            position[b_axis] = &self.sin_theta * hit.p[a_axis] + &self.cos_theta * hit.p[b_axis];
            normal[a_axis] =
                &self.cos_theta * hit.normal[a_axis] - &self.sin_theta * hit.normal[b_axis];
            normal[b_axis] =
                &self.sin_theta * hit.normal[a_axis] + &self.cos_theta * hit.normal[b_axis];

            hit.p = position;
            hit.set_face_normal(&rotate_ray, normal);
            hit
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.rbox.clone()
    }
}
