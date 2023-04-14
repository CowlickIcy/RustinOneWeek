use std::mem::swap;

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
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(resolution);
    for _ in 0..resolution {
        v.push(Vector3::random_in_unit_sphere());
    }
    v
}

pub fn permute(p: &mut Vec<usize>, resolution: usize) {
    let mut rng = rand::thread_rng();
    for iter in (resolution - 1)..=0 {
        let mut target = rng.gen_range(0..=iter);
        let mut tmp = p[iter];
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
            perm_z: perlin_generate_perm(resolution)
        }
    }

    
}
