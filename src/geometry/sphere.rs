use super::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Sphere<M: Material> {
    center: Point,
    radius: f64,
    mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point, radius: f64, mat: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            mat,
        }
    }

    pub fn get_sphere_uv(p: &Point) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }

    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
        let phi = 2.0 * f64::consts::PI * r1;
        let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
        let y = phi.sin() * (1.0 - z.powi(2)).sqrt();
        Vector3::new(x, y, z)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.dir().length_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord {
            p: p,
            normal: Vector3::default(),
            t: root,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: &self.mat,
        };
        rec.set_face_normal(r, outward_normal);
        let (u, v) = Sphere::<M>::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let min = self.center - Vector3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vector3::new(self.radius, self.radius, self.radius);
        Some(Aabb { min, max })
    }

    fn pdf_value(&self, o: Point, v: Vector3) -> f64 {
        if let Some(_hit) = self.hit(&Ray::new(o, v, 0.0), 0.001, f64::INFINITY) {
            let cos_theta_max =
                (1.0 - self.radius.powi(2) / (self.center - o).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: Vector3) -> Vector3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::build_from_w(&direction);
        uvw.local(&Sphere::<M>::random_to_sphere(self.radius, distance_squared))
    }
}

#[derive(Clone, Copy)]
pub struct MovingSphere<M: Material> {
    center0: Point,
    center1: Point,
    time0: f64,
    time1: f64,
    radius: f64,
    mat: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point,
        center1: Point,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: M,
    ) -> MovingSphere<M> {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat,
        }
    }

    pub fn get_center(&self, t: f64) -> Point {
        self.center0 + ((t - self.time0) / (t / self.time1)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - MovingSphere::get_center(self, r.time());
        let a = r.dir().length_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - MovingSphere::get_center(self, r.time())) / self.radius;
        let mut rec = HitRecord {
            p: p,
            normal: Vector3::default(),
            t: root,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: &self.mat,
        };
        rec.set_face_normal(r, outward_normal);
        let (u, v) = Sphere::<M>::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let min0 = self.center0 - Vector3::new(self.radius, self.radius, self.radius);
        let max0 = self.center0 + Vector3::new(self.radius, self.radius, self.radius);
        let min1 = self.center1 - Vector3::new(self.radius, self.radius, self.radius);
        let max1 = self.center1 + Vector3::new(self.radius, self.radius, self.radius);

        let box0 = Aabb::new(min0, max0);
        let box1 = Aabb::new(min1, max1);

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
