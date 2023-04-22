use super::*;
use rand::Rng;

fn random_cosine_direction() -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f64>();
    let r2 = rng.gen::<f64>();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vector3::new(x, y, z)
}

fn spherical_direction(sin_theta: f64, cos_theta: f64, sin_phi: f64, cos_phi: f64) -> Vector3 {
    Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
}

fn gtr_1_direction(r_in: Vector3, clearcoat_gloss: f64) -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let a = lerp(0.1, 0.001, clearcoat_gloss);
    let a2 = a * a;
    let cos_theta = f64::sqrt(f64::max(0.001, (1.0 - a2.powf(1.0 - r1)) / (1.0 - a2)));
    let sin_theta = f64::sqrt(f64::max(0.001, 1.0 - cos_theta * cos_theta));
    let phi = f64::consts::PI * 2.0 * r2;
    let wh = spherical_direction(sin_theta, cos_theta, f64::sin(phi), f64::cos(phi));

    Vector3::reflect(r_in, wh)
}

fn gtr_2_aniso_direction(r_in: Vector3, roughness: f64, anisotropic: f64) -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let aspect = (1.0 - anisotropic * 0.9).sqrt();
    let ax = (roughness.powi(2) / aspect).max(0.001);
    let ay = (roughness.powi(2) * aspect).max(0.001);
    let mut phi = f64::atan(ay / ax * f64::tan(2.0 * f64::consts::PI * r2 + 0.5 * f64::consts::PI));
    if r2 > 0.5 {
        phi += f64::consts::PI;
    }
    let sin_phi = f64::sin(phi);
    let cos_phi = f64::cos(phi);
    let ax_2 = ax * ax;
    let ay_2 = ay * ay;
    let a2 = 1.0 / (cos_phi * cos_phi / ax_2 + sin_phi * sin_phi / ay_2);
    let tan_theta_2 = a2 * r1 / (1.0 - r1);
    let cos_theta = 1.0 / (1.0 + tan_theta_2).sqrt();
    let sin_theta = f64::sqrt(f64::max(0.001, 1.0 - cos_theta * cos_theta));
    let wh = spherical_direction(sin_theta, cos_theta, f64::sin(phi), f64::cos(phi));

    Vector3::reflect(r_in, wh)
}

pub enum PDF<'a> {
    BRDF {
        uvw: ONB,
        r_in: Vector3,
        roughness: f64,
        anisotropic: f64,
        clearcoat: f64,
        clearcoat_gloss: f64,
    },
    Cosine {
        uvw: ONB,
    },
    Hittable {
        origin: Point,
        hittable: &'a Box<dyn Hittable>,
    },
    Mixture {
        p0: &'a PDF<'a>,
        p1: &'a PDF<'a>,
    },
}

impl<'a> PDF<'a> {
    pub fn brdf_pdf(
        w: Vector3,
        r_in: Vector3,
        roughness: f64,
        anisotropic: f64,
        clearcoat: f64,
        clearcoat_gloss: f64,
    ) -> PDF<'a> {
        PDF::BRDF {
            uvw: ONB::build_from_w(&w),
            r_in,
            roughness,
            anisotropic,
            clearcoat,
            clearcoat_gloss,
        }
    }

    pub fn cosine_pdf(w: Vector3) -> PDF<'a> {
        PDF::Cosine {
            uvw: ONB::build_from_w(&w),
        }
    }

    pub fn hittable_pdf(origin: Point, hittable: &'a Box<dyn Hittable>) -> PDF<'a> {
        PDF::Hittable { origin, hittable }
    }

    pub fn mixture_pdf(p0: &'a PDF, p1: &'a PDF) -> PDF<'a> {
        PDF::Mixture { p0, p1 }
    }

    pub fn value(&self, r_out: Vector3) -> f64 {
        match self {
            PDF::BRDF {
                uvw,
                r_in,
                roughness,
                anisotropic,
                clearcoat,
                clearcoat_gloss,
            } => {
                let cosine = r_out.normalize().dot(uvw.w());
                if cosine < 0.0 {
                    return 0.0;
                }
                let l = -r_in.normalize();
                let v = r_out.normalize();
                let n = uvw.w();
                let x = uvw.u();
                let y = uvw.v();
                let n_dot_l = n.dot(l);
                let h = (l + v).normalize();
                let n_dot_h = n.dot(h);

                if n_dot_h <= 0.0 {
                    return 0.0;
                }
                // diffuse
                let diffuse_pdf = cosine / PI;

                // specular
                let aspect = (1.0 - anisotropic * 0.9).sqrt();
                let ax = (roughness.powi(2) / aspect).max(0.0001);
                let ay = (roughness.powi(2) * aspect).max(0.0001);
                let specular_pdf = gtr_2_anisotropic(n_dot_h, h.dot(x), h.dot(y), ax, ay)
                    * f64::abs(n_dot_h)
                    * 0.25
                    / n_dot_l;

                // clearcoat
                let clearcoat_pdf =
                    gtr_1(n_dot_h, lerp(0.1, 0.001, *clearcoat_gloss)) * f64::abs(n_dot_h) * 0.25
                        / n_dot_l;

                (diffuse_pdf + specular_pdf + clearcoat_pdf) / 3.0
            }
            PDF::Cosine { uvw } => {
                let cosine = r_out.normalize().dot(uvw.w());
                if cosine > 0.0 {
                    cosine / PI
                } else {
                    0.0
                }
            }
            PDF::Hittable { origin, hittable } => hittable.pdf_value(*origin, r_out),
            PDF::Mixture { p0, p1 } => 0.5 * p0.value(r_out) + 0.5 * p1.value(r_out),
        }
    }

    pub fn generate(&self) -> Vector3 {
        match self {
            PDF::BRDF {
                uvw,
                r_in,
                roughness,
                anisotropic,
                clearcoat,
                clearcoat_gloss,
            } => {
                let rng = rand::thread_rng().gen_range(0.0..1.0);
                if rng < 0.333 {
                    uvw.local(&random_cosine_direction())
                } else if rng < 0.666 {
                    uvw.local(&&gtr_1_direction(*r_in, *clearcoat_gloss))
                } else {
                    uvw.local(&gtr_2_aniso_direction(*r_in, *roughness, *anisotropic))
                }
            }
            PDF::Cosine { uvw } => uvw.local(&random_cosine_direction()),
            PDF::Hittable { origin, hittable } => hittable.random(*origin),
            PDF::Mixture { p0, p1 } => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    p0.generate()
                } else {
                    p1.generate()
                }
            }
        }
    }
}
