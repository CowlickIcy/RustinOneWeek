use super::*;
pub struct Ray {
    origin: Point,
    dir: Vector3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vector3, dir: Vector3, time: f64) -> Ray {
        Ray { origin, dir, time }
    }
    pub fn at(self, t: f64) -> Point {
        let ret = self.dir * t + self.origin;
        ret
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn dir(&self) -> Vector3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
