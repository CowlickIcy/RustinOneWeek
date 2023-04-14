use super::*;
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    min: Point,
    max: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Aabb {
        Aabb { min, max }
    }

    pub fn get_min(&self) -> Point {
        self.min
    }

    pub fn get_max(&self) -> Point {
        self.max
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
        Aabb {
            min: Vector3 {
                e: [
                    f64::min(box0.get_min().x(), box1.get_min().x()),
                    f64::min(box0.get_min().y(), box1.get_min().y()),
                    f64::min(box0.get_min().z(), box1.get_min().z()),
                ],
            },
            max: Vector3 {
                e: [
                    f64::max(box0.get_max().x(), box1.get_max().x()),
                    f64::max(box0.get_max().y(), box1.get_max().y()),
                    f64::max(box0.get_max().z(), box1.get_max().z()),
                ],
            },
        }
    }
}
