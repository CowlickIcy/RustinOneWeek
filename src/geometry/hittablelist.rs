use super::*;

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

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new_without_param();
        let mut hit_anything = false;
        let mut closest_so_for = t_max;
        for i in 0..self.objects.len() {
            if self.objects[i].hit(&r, t_min, t_max, &mut tmp_rec) == true {
                hit_anything = true;
                closest_so_for = tmp_rec.t;
                *rec = tmp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut tmp_box = Aabb::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        let mut first_box = true;
        for i in 0..self.objects.len() {
            if self.objects[i].bounding_box(time0, time1, &mut tmp_box) {
                return false;
            }
            if first_box == true {
                *output_box = tmp_box;
            } else {
                *output_box = Aabb::surrounding_box(&output_box, &tmp_box);
            }
            first_box = false;
        }
        true
    }
}
