use super::vector::*;

type Point3 = Vector3;
struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
    pub time: f64,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        let ret = self.dir.mul_with_num(t).add(self.origin);
        ret
    }
}
