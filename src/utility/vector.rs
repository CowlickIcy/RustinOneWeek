#![allow(dead_code)]

//

use self::vector::Vector3;

mod vector {
    use rand::Rng;

    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    type Point = Vector3;
    type Color = Vector3;

    impl Vector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3 { x: x, y: y, z: z }
        }

        pub fn unit_vectors(&self) -> Vector3 {
            Vector3
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
    }
}

// ops redef

impl Mul for Vector3 {
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div for Vector3 {
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Add for Vector3 {
    fn add(self, rhs: Self) -> Vector3 {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    fn sub(self, rhs: Self) -> Vector3 {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
