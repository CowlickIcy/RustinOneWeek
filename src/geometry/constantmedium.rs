use super::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct ConstantMedium<H: Hittable, M: Material> {
    boundary: H,
    phase_func: M,
    neg_inv_density: f64,
}

impl<H: Hittable, M: Material> ConstantMedium<H, M> {
    pub fn new(boundary: H, phase_func: M, neg_inv_density: f64) -> ConstantMedium<H, M> {
        ConstantMedium {
            boundary,
            phase_func,
            neg_inv_density,
        }
    }
}

impl<H: Hittable, M: Material> Hittable for ConstantMedium<H, M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();

        if let Some(mut hit1) = self.boundary.hit(r, -f64::MAX, f64::MAX) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, f64::MAX) {
                hit1.t = hit1.t.max(t_min);
                hit2.t = hit2.t.min(t_max);

                if hit1.t < hit2.t {
                    let ray_length = r.dir().length();
                    let distance_inside_boundary = (hit2.t - hit1.t) / ray_length;
                    let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / ray_length;
                        return Some(HitRecord {
                            p: r.at(t),
                            normal: Vector3::new(1.0, 0.0, 0.0),
                            t,
                            u: 0.0,
                            v: 0.0,
                            front_face: false,
                            mat: &self.phase_func,
                        });
                        // return None;
                    }
                }
            }
        }
        None
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
