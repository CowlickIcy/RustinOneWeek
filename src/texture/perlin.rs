use super::*;
use rand::Rng;

fn generate_single(resolution: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(resolution);
    for _ in 0..resolution {
        v.push(rng.gen_range(0.0..=1.0))
    }
    v
}

fn generate_rgb(resolution: usize) -> Vec<Vector3> {
    let mut v = Vec::with_capacity(resolution);
    for _ in 0..resolution {
        v.push(Vector3::random_in_unit_sphere());
    }
    v
}

pub fn permute(p: &mut Vec<usize>, resolution: usize) {
    let mut rng = rand::thread_rng();
    for iter in (resolution - 1)..=0 {
        let target = rng.gen_range(0..=iter);
        let tmp = p[iter];
        p[iter] = p[target];
        p[target] = tmp;
    }
}

pub fn perlin_generate_perm(resolution: usize) -> Vec<usize> {
    let mut p = Vec::with_capacity(resolution);
    for iter in 0..resolution {
        p.push(iter);
    }

    permute(&mut p, resolution);
    p
}

// trillinear interpolate
pub fn perlin_interp(c: &mut [[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for iter in 0..=2 {
        for jter in 0..=2 {
            for kter in 0..=2 {
                let weight_v = Vector3::new(u - iter as f64, v - jter as f64, ww - kter as f64);
                accum += (iter as f64 * uu + (1.0 - iter as f64) * (1.0 - uu))
                    * (jter as f64 * vv + (1.0 - jter as f64) * (1.0 - vv))
                    * (kter as f64 * ww + (1.0 - kter as f64) * (1.0 - ww))
                    * weight_v.dot(c[iter][jter][kter]);
            }
        }
    }
    accum
}

#[derive(Debug, Clone)]
pub struct Perlin {
    resolution: usize,
    rand_vec: Vec<Vector3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(resolution: usize) -> Perlin {
        Perlin {
            resolution: resolution,
            rand_vec: generate_rgb(resolution),
            perm_x: perlin_generate_perm(resolution),
            perm_y: perlin_generate_perm(resolution),
            perm_z: perlin_generate_perm(resolution),
        }
    }

    pub fn noise(&self, p: &Point, scale: f64) -> f64 {
        let u = p.x() * scale - (p.x() * scale).floor();
        let v = p.y() * scale - (p.y() * scale).floor();
        let w = p.z() * scale - (p.z() * scale).floor();

        let i = (p.x() * scale).floor() as usize;
        let j = (p.y() * scale).floor() as usize;
        let k = (p.z() * scale).floor() as usize;

        let mut c = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..=1 {
            for dj in 0..=1 {
                for dk in 0..=1 {
                    c[di][dj][dk] = self.rand_vec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }
        perlin_interp(&mut c, u, v, w)
    }
}
