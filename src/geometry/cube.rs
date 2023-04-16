use super::*;

pub struct Cube {
    cube_min: Point,
    cube_max: Point,
    sides: HittableList,
}

impl Cube {
    pub fn new<M: Material + Clone + 'static>(p0: Point, p1: Point, mat: M) -> Cube {
        let mut sides = HittableList::default();

        sides.add(AARect::new(
            Plane::XZ,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        ));
        sides.add(AARect::new(
            Plane::XY,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        ));
        sides.add(AARect::new(
            Plane::XZ,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        ));
        sides.add(AARect::new(
            Plane::XZ,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        ));
        sides.add(AARect::new(
            Plane::YZ,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        ));
        sides.add(AARect::new(
            Plane::YZ,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat,
        ));
        Cube {
            cube_min: p0,
            cube_max: p1,
            sides: sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.cube_min,
            max: self.cube_max,
        })
    }
}
