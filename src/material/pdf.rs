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

    // pub fn hittable_pdf()
}
