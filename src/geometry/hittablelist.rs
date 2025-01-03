use rand::seq::SliceRandom;

use super::*;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_for = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_for) {
                closest_so_for = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self.objects.first() {
            Some(first) => match first.bounding_box(time0, time1) {
                Some(bbox) => self.objects.iter().skip(1).try_fold(bbox, |acc, hittable| {
                    match hittable.bounding_box(time0, time1) {
                        Some(bbox) => Some(Aabb::surrounding_box(&acc, &bbox)),
                        _ => None,
                    }
                }),
                _ => None,
            },
            _ => None,
        }
    }
    fn pdf_value(&self, o: Point, v: Vector3) -> f64 {
        self.objects.iter().map(|h| h.pdf_value(o, v)).sum::<f64>() / self.objects.len() as f64
    }

    fn random(&self, o: Vector3) -> Vector3 {
        self.objects.choose(&mut rand::thread_rng()).unwrap().random(o)
    }
}
