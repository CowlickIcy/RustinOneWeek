use super::*;

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

                if n_dot_h < 0.0 {
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
}
