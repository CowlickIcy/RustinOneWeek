use super::*;

pub struct Translate {
    pub hptr: Box<dyn Hittable>,
    pub offset: Vector3,
}
impl Translate {
    pub fn new(hptr: Box<dyn Hittable>, offset: Vector3) -> Translate {
        Translate {
            hptr: hptr,
            offset: offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin - self.offset, r.dir, r.time);
        if self.hptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;
        let normal = rec.normal;
        rec.set_face_normal(&moved_r, &normal);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.hptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = Aabb::new(
            output_box.get_min() + self.offset,
            output_box.get_max() + self.offset,
        );
        true
    }
}
