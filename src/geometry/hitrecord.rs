use super::*;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector3,
    // pub mat: Box<dyn Material>,
    pub t: f64, // hit time
    pub u: f64,
    pub v: f64,
    pub front_face: bool,

    pub mat: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        if self.front_face == true {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}


