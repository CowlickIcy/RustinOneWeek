use super::*;
use rand::Rng;
use std::marker::Sync;

#[derive(Debug, Copy, Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Lambertian<T> {
        Lambertian { albedo }
    }
}

impl<T: Texture + Sync> Material for Lambertian<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vector3::random_in_unit_sphere().normalize();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scatted = Ray::new(rec.p, scatter_direction, r_in.time());

        Some((self.albedo.get_color(rec.u, rec.v, &rec.p), scatted))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metallic {
    albedo: Color,
    fuzz: f64,
}

impl Metallic {
    pub fn new(albedo: Color, fuzz: f64) -> Metallic {
        Metallic { albedo, fuzz }
    }
}

impl Material for Metallic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.dir().reflect(rec.normal).normalize();
        let scattered = Ray::new(
            rec.p,
            reflected + Vector3::random_in_unit_sphere() * self.fuzz,
            r_in.time(),
        );
        if scattered.dir().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

fn reflectance(cosine: f64, ref_index: f64) -> f64 {
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut rng = rand::thread_rng();
        // let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.dir().normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction, r_in.time());
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight<T: Texture> {
    albedo: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(albedo: T) -> DiffuseLight<T> {
        DiffuseLight { albedo }
    }
}

impl<T: Texture + Sync> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, rec: &HitRecord) -> Color {
        if rec.front_face {
            self.albedo.get_color(rec.u, rec.v, &rec.p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Isotropic<T> {
        Isotropic { albedo }
    }
}

impl<T: Texture + Sync> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vector3::random_in_unit_sphere(), r_in.time());
        Some((self.albedo.get_color(rec.u, rec.v, &rec.p), scattered))
    }
}
