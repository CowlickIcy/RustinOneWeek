#![allow(dead_code)]
use rand::Rng;
use std::f64;
use std::ops;

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3 {
    e: [f64; 3],
}

// implement
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { e: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }

    pub fn y(self) -> f64 {
        self[1]
    }

    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vector3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [
                self[1] * other[2] + self[2] * other[1],
                self[2] * other[0] + self[0] * other[2],
                self[0] * other[0] + self[1] * other[1],
            ],
        }
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vector3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const E: f64 = 1e-8;
        f64::abs(self[0]) < E && f64::abs(self[1]) < E && f64::abs(self[2]) < E
    }

    pub fn random(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3 {
            e: [
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
                rng.gen_range(min..=max),
            ],
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random(-1.0, 1.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vector3) -> Vector3 {
        let p = Vector3::random_in_unit_sphere();
        if p.dot(normal) > 0.0 {
            p
        } else {
            -p
        }
    }

    /// ### self : view vector
    pub fn reflect(self, normal: Vector3) -> Vector3 {
        self - normal * self.dot(normal) * 2.0
    }

    /// ### self : uv vector
    pub fn refract(self, normal: Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = -self.dot(normal).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared().abs().sqrt());
        r_out_perp + r_out_parallel
    }
}

// ops
impl ops::Index<usize> for Vector3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl ops::Mul<&Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3 {
            e: [self * other[0], self * other[1], self * other[2]],
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            e: [self * other[0], self * other[1], self * other[2]],
        }
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}
impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vector3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        }
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        *self = Vector3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        }
    }
}
