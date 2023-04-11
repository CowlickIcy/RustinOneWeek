use super::vector::*;

type Point3 = Vector3;
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3, time: f64) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
            time: time,
        }
    }
    pub fn at(self, t: f64) -> Point3 {
        let ret = self.dir.mul_with_num(t).add(self.origin);
        ret
    }
}
