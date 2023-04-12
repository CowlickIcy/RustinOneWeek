use super::*;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector3,
    pub mat: Box<dyn Material>,
    pub t: f64, // hit time
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point,
        normal: Vector3,
        mat: Box<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
    pub fn new_without_param() -> HitRecord {
        HitRecord {
            p: Point::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            mat: Box::new(BaseMaterial::new(0)),
            t: 0.0,
            u:0.0,
            v:0.0,
            front_face: true,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        if self.front_face == true {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}
