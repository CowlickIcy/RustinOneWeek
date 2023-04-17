use super::*;
use std::path::Path;
use tobj;

use crate::utility::Point;

#[derive(Debug, Clone)]
pub struct Triangle<M: Material> {
    vectices: [Point; 3],
    mat: M,
}

impl<M: Material> Triangle<M> {
    pub fn new(vectices: [Point; 3], mat: M) -> Triangle<M> {
        Triangle { vectices, mat }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let e1 = self.vectices[1] - self.vectices[0];
        let e2 = self.vectices[2] - self.vectices[0];

        let s = r.origin() - self.vectices[0];
        let s1 = r.dir().cross(e2);
        let s2 = s.cross(e1);
        let s1_e1 = s1.dot(e1);

        let t = s2.dot(e2) / s1_e1;
        let b1 = s1.dot(s) / s1_e1;
        let b2 = s2.dot(r.dir()) / s1_e1;

        if t < t_min || t > t_max {
            None
        } else {
            if b1 < 0.0 || b2 > 0.0 || (1.0 - b1 - b2) < 0.0 {
                None
            } else {
                let p = r.at(t);
                let normal = e1.cross(e2).normalize();
                let mut rec = HitRecord {
                    p,
                    normal,
                    t,
                    u: b1,
                    v: b2,
                    front_face: false,
                    mat: &self.mat,
                };
                rec.set_face_normal(r, normal);
                Some(rec)
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let min_x = self.vectices[0]
            .x()
            .min(self.vectices[1].x().min(self.vectices[2].x()));
        let min_y = self.vectices[0]
            .y()
            .min(self.vectices[1].y().min(self.vectices[2].y()));
        let min_z = self.vectices[0]
            .z()
            .min(self.vectices[1].z().min(self.vectices[2].z()));
        let max_x = self.vectices[0]
            .x()
            .min(self.vectices[1].x().min(self.vectices[2].x()));
        let max_y = self.vectices[0]
            .y()
            .min(self.vectices[1].y().min(self.vectices[2].y()));
        let max_z = self.vectices[0]
            .z()
            .min(self.vectices[1].z().min(self.vectices[2].z()));

        let min = Vector3::new(min_x, min_y, min_z);
        let max = Vector3::new(max_x, max_y, max_z);

        Some(Aabb { min, max })
    }
}

pub struct Mesh {
    pub tris: HittableList,
}

impl Mesh {
    pub fn new<M: Material + Copy + Clone + 'static>(
        positions: Vec<Vector3>,
        indices: Vec<u32>,
        mat: M,
    ) -> Mesh {
        let mut tris = HittableList::default();

        for iter in 0..indices.len() / 3 {
            let vertices = [
                positions[indices[iter * 3] as usize],
                positions[indices[iter * 3 + 1] as usize],
                positions[indices[iter * 3 + 2] as usize],
            ];
            tris.add(Triangle::new(vertices, mat));
        }
        Mesh { tris }
    }
    pub fn load_obj<'a, P: AsRef<Path>, M: Material + Copy + Clone + 'static>(
        path: P,
        mat: M,
    ) -> Result<Mesh, String> {
        let models = match tobj::load_obj(path.as_ref(), &tobj::OFFLINE_RENDERING_LOAD_OPTIONS) {
            Ok((models, _)) => {
                let m = &models[0];
                let mesh = &m.mesh;
                let tri_positions = mesh
                    .positions
                    .chunks(3)
                    .map(|p| Point::new(p[0] as f64, p[1] as f64, p[2] as f64))
                    .collect();
                let tri_indices = &mesh.indices;

                Mesh::new(tri_positions, tri_indices.to_vec(), mat)
            }
            Err(err) => return Err(format!("Failed to load obj file :{}", err)),
        };
        Ok(models)
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.tris.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.tris.bounding_box(time0, time1)
    }
}
