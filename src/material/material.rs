use super::*;
use rand::Rng;

pub fn mon_to_linear(x: Color) -> Color {
    Vector3::new(x.x().powf(2.2), x.y().powf(2.2), x.z().powf(2.2))
}

pub fn schlick_fresnel(x: f64) -> f64 {
    let y = (1.0 - x).clamp(0.0, 1.0);
    let y_2 = y * y;
    y_2 * y_2 * y
}

pub fn smith_ggx_anisotropic(cos_theta: f64, cos_phi: f64, sin_phi: f64, ax: f64, ay: f64) -> f64 {
    let tan_theta_2 = (1.0 - cos_theta.powi(2)) / cos_theta.powi(2).max(0.001);
    let lambda_s =
        -0.5 + 0.5 * (1.0 + (ax * cos_phi).powi(2) + (ay * sin_phi).powi(2) * tan_theta_2).sqrt();
    1.0 / (1.0 + lambda_s)
}

pub fn smith_ggx(cos_theta: f64, a: f64) -> f64 {
    smith_ggx_anisotropic(cos_theta, 1.0, 0.0, a, a)
}

pub fn gtr_2_anisotropic(cos_theta: f64, cos_phi: f64, sin_phi: f64, ax: f64, ay: f64) -> f64 {
    1.0 / (PI
        * ax
        * ay
        * (1.0 - cos_theta.powi(2))
        * ((cos_phi / ax).powi(2) + (sin_phi / ay).powi(2))
        + cos_theta.powi(2))
    .powi(2)
}

pub fn gtr_1(n_dot_h: f64, a: f64) -> f64 {
    (a.powi(2) - 1.0) / 2.0 * PI * a.ln() * ((a * n_dot_h).powi(2) + (1.0 - n_dot_h * n_dot_h))
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

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

#[derive(Debug, Clone)]
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
        self.albedo.get_color(rec.u, rec.v, &rec.p)
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

#[derive(Debug, Clone, Copy)]
pub struct PBR<T: Texture> {
    base_color: T,        // 基本颜色
    metallic: f64,        // 漫反射和次表面散射的lerp因子
    subsurface: f64,      // 金属度
    specular: f64,        // 高光度
    roughness: f64,       // 高光色向基本色靠拢的程度
    specular_tint: f64,   // 粗糙度
    anisotropic: f64,     // 各向异性
    sheen: f64,           // 光泽度，纺织物边缘明亮程度
    sheen_tint: f64,      // sheen向基本色靠拢程度
    clearcoat: f64,       // 清漆高光
    clearcoat_gloss: f64, // 清漆光滑程度
}

impl<T: Texture> PBR<T> {
    pub fn new(
        base_color: T,
        metallic: f64,
        subsurface: f64,
        specular: f64,
        roughness: f64,
        specular_tint: f64,
        anisotropic: f64,
        sheen: f64,
        sheen_tint: f64,
        clearcoat: f64,
        clearcoat_gloss: f64,
    ) -> PBR<T> {
        PBR {
            base_color,
            metallic,
            subsurface,
            specular,
            roughness,
            specular_tint,
            anisotropic,
            sheen,
            sheen_tint,
            clearcoat,
            clearcoat_gloss,
        }
    }
}

impl<T: Texture + Sync> Material for PBR<T> {
    fn brdf(&self, r_in: &Ray, r_out: &Ray, rec: &HitRecord) -> Vector3 {
        let l = -r_in.dir().normalize();
        let v = r_out.dir().normalize();
        let onb = ONB::build_from_w(&rec.normal);

        let n = onb.w();
        let x = onb.u();
        let y = onb.v();

        let n_dot_v = n.dot(v); // theta_o
        let n_dot_l = n.dot(l); // theta_i

        if n_dot_l < 0.0 || n_dot_v < 0.0 {
            return Vector3::default();
        }

        let h = (l + v).normalize();
        let n_dot_h = n.dot(h);
        let l_dot_h = l.dot(h);

        // color
        let base_color = self.base_color.get_color(rec.u, rec.v, &rec.p);
        let linear_color = mon_to_linear(base_color);
        let luminance_color =
            0.2126 * linear_color.x() + 0.7152 * linear_color.y() + 0.0722 * linear_color.z();
        let c_tint = if luminance_color > 0.0 {
            linear_color / luminance_color
        } else {
            Vector3::one()
        };

        // aniso
        let aspect = (1.0 - self.anisotropic * 0.9).sqrt();
        let ax = (self.roughness.powi(2) / aspect).max(0.0001);
        let ay = (self.roughness.powi(2) * aspect).max(0.0001);

        // fresnel
        let fresnel_l = schlick_fresnel(n_dot_l);
        let fresnel_v = schlick_fresnel(n_dot_v);
        let fresnel_h = schlick_fresnel(n_dot_h);

        // diffuse
        let fresnel_90 = 0.5 + 2.0 * l_dot_h.powi(2) * self.roughness;
        let fresnel_diffuse = self.base_color.get_color(rec.u, rec.v, &rec.p) / PI
            * (1.0 + (fresnel_90 - 1.0) * fresnel_l)
            * (1.0 + (fresnel_90 - 1.0) * fresnel_v);

        // bssdf
        let fresnel_90 = l_dot_h.powi(2) * self.roughness;
        let fresnel_ss =
            (1.0 + (fresnel_90 - 1.0) * fresnel_l) * (1.0 + (fresnel_90 - 1.0) * fresnel_v);
        let fresnel_subsurface = 1.25
            * self.base_color.get_color(rec.u, rec.v, &rec.p)
            * (fresnel_ss * (1.0 / (n_dot_v + n_dot_l) - 0.5) + 0.5);

        // specular
        let c_st = Vector3::one().lerp(c_tint, self.sheen_tint);
        let c_specular = (0.08 * self.specular * c_st).lerp(base_color, self.metallic);
        let specular_f = c_specular + (Vector3::one() - c_specular) * fresnel_h;
        let specular_g = smith_ggx_anisotropic(n_dot_l, l.dot(x), l.dot(y), ax, ay)
            * smith_ggx_anisotropic(n_dot_v, v.dot(x), v.dot(y), ax, ay);
        let specular_d = gtr_2_anisotropic(n_dot_h, h.dot(x), h.dot(y), ax, ay);

        // sheen
        let c_sht = Vector3::one().lerp(c_tint, self.sheen_tint);
        let fresnel_sheen = c_sht * self.sheen * fresnel_h;

        // clearcoat
        let clearcoat_f = lerp(fresnel_h, 1.0, 0.04);
        let clearcoat_g = smith_ggx(n_dot_l, 0.25) * smith_ggx(n_dot_v, 0.25);
        let clearcoat_d = gtr_1(n_dot_h, lerp(0.1, 0.01, self.clearcoat_gloss));

        (1.0 - self.metallic)
            * (linear_color / PI * fresnel_diffuse.lerp(fresnel_subsurface, self.subsurface)
                + fresnel_sheen)
            + specular_d * specular_g * specular_f / (4.0 * n_dot_v * n_dot_l)
            + self.clearcoat * 0.25 * Vector3::one() * clearcoat_d * clearcoat_f * clearcoat_g
                / (4.0 * n_dot_v * n_dot_l)
    }
}
