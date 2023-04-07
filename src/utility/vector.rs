#![allow(dead_code)]
use rand::Rng;
use std::ops;

#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

type Point = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn unit_vector(&self) -> Vector3 {
        Vector3::div_with_num(self, Vector3::length(self))
    }

    pub fn origin() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(Vector3::length_squared(self))
    }

    pub fn near_zero(&self) -> bool {
        let e = 1e-8f64;
        f64::abs(self.x) < e && f64::abs(self.y) < e && f64::abs(self.z) < e
    }

    // ops
    pub fn add(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub fn mul_with_num(&self, num: f64) -> Vector3 {
        Vector3 {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }

    pub fn div_with_num(&self, num: f64) -> Vector3 {
        if num >= 0.0 {
            Vector3 {
                x: self.x / num,
                y: self.y / num,
                z: self.z / num,
            }
        } else {
            panic!("div num can not be zero!");
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let mut rng = rand::thread_rng();
            let p = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
            if Vector3::length_squared(&p) >= 1.0 {
            } else {
                return p;
            }
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random(-1.0, 1.0);
            if Vector3::length_squared(&p) >= 1.0 {
            } else {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(&self, normal: &Vector3) -> Vector3 {
        let p = Vector3::random_in_unit_sphere();
        if Vector3::dot(self, normal) > 0.0 {
            p
        } else {
            -p
        }
    }

    pub fn reflect(view: &Vector3, normal: &Vector3) -> Vector3 {
        let p = normal.mul_with_num(view.mul_with_num(2.0).dot(normal));
        Vector3::new(view.x - p.x, view.y - p.y, view.z - p.z)
    }

    pub fn refract(uv: &Vector3, normal: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = f64::min(uv.dot(normal), 1.0);
        let r_out_perp: Vector3 = uv
            .add(normal.mul_with_num(cos_theta))
            .mul_with_num(etai_over_etat);
        let r_out_parallel =
            normal.mul_with_num(-f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())));
        r_out_parallel + r_out_perp
    }
}

// ops redef
impl ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Vector3 {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Vector3 {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Self::new(-self.x, -self.y, -self.z)
    }
}
