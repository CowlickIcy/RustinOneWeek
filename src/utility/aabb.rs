use super::*;
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Aabb {
        Aabb { min, max }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for iter in 0..=2 {
            let inv_d = 1.0 / r.dir()[iter];
            let t0 = (self.min[iter] - r.origin()[iter]) * inv_d;
            let t1 = (self.max[iter] - r.origin()[iter]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            if t_max.min(t0) <= t_min.max(t1) {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let min = Vector3::new(
            f64::min(box0.min.x(), box1.min.x()),
            f64::min(box0.min.y(), box1.min.y()),
            f64::min(box0.min.z(), box1.min.z()),
        );

        let max = Vector3::new(
            f64::max(box0.max.x(), box1.max.x()),
            f64::max(box0.max.y(), box1.max.y()),
            f64::max(box0.max.z(), box1.max.z()),
        );
        Aabb { min, max }
    }
}
