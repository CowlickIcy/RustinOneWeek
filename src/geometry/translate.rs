use super::*;

#[derive(Debug, Clone)]
pub struct Translate<H: Hittable> {
    hptr: H,
    offset: Vector3,
}
impl<H: Hittable> Translate<H> {
    pub fn new(hptr: H, offset: Vector3) -> Translate<H> {
        Translate { hptr, offset }
    }
}

impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.dir(), r.time());
        self.hptr.hit(&moved_r, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.hptr.bounding_box(time0, time1).map(|mut aabb| {
            aabb.min += self.offset;
            aabb.max += self.offset;
            aabb
        })
    }
}
