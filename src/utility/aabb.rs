use super::*;
pub struct Aabb {
    min: Point,
    max: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Aabb {
        Aabb { min: min, max: max }
    }

    pub fn get_min(&self) -> Point {
        self.min
    }

    pub fn get_max(&self) -> Point {
        self.max
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t0 = f64::min(
            (self.get_min().x - r.origin.x) / r.dir.x,
            (self.get_max().x - r.origin.x) / r.dir.x,
        );
        let mut t1 = f64::max(
            (self.get_min().x - r.origin.x) / r.dir.x,
            (self.get_max().x - r.origin.x) / r.dir.x,
        );
        let mut t_min = f64::max(t0, t_min);
        let mut t_max = f64::min(t1, t_max);
        t0 = f64::min(
            (self.get_min().y - r.origin.y) / r.dir.y,
            (self.get_max().y - r.origin.y) / r.dir.y,
        );
        t1 = f64::max(
            (self.get_min().y - r.origin.y) / r.dir.y,
            (self.get_max().y - r.origin.y) / r.dir.y,
        );
        t_min = f64::max(t0, t_min);
        t_max = f64::min(t1, t_max);
        t0 = f64::min(
            (self.get_min().z - r.origin.z) / r.dir.z,
            (self.get_max().z - r.origin.z) / r.dir.z,
        );
        t1 = f64::max(
            (self.get_min().z - r.origin.z) / r.dir.z,
            (self.get_max().z - r.origin.z) / r.dir.z,
        );
        t_min = f64::max(t0, t_min);
        t_max = f64::min(t1, t_max);
        if t_max <= t_min {
            return false;
        } else {
            return true;
        }
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            min: Vector3 {
                x: f64::min(box0.get_min().x, box1.get_min().x),
                y: f64::min(box0.get_min().y, box1.get_min().y),
                z: f64::min(box0.get_min().z, box1.get_min().z),
            },
            max: Vector3 {
                x: f64::min(box0.get_min().x, box1.get_min().x),
                y: f64::min(box0.get_min().y, box1.get_min().y),
                z: f64::min(box0.get_min().z, box1.get_min().z),
            },
        }
    }
}
