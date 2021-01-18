use std::fmt;

use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::vec::{vec3, Vec3};

const SEED: u64 = 0;
const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    pub seed: u64,
    pub perm_x: [u32; POINT_COUNT],
    pub perm_y: [u32; POINT_COUNT],
    pub perm_z: [u32; POINT_COUNT],
    pub rndvec: [Vec3; POINT_COUNT],
}

impl fmt::Debug for Perlin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Perlin").field("seed", &self.seed).finish()
    }
}

impl Perlin {
    pub fn new(seed: u64) -> Perlin {
        let mut rng = SmallRng::seed_from_u64(SEED);
        let mut rnd: [Vec3; POINT_COUNT] = [Vec3::zero(); POINT_COUNT];
        let mut perm_x: [u32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_y: [u32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_z: [u32; POINT_COUNT] = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            //rnd[i] = Vec3::random_unit_vector(&mut rng);
            rnd[i] = Vec3::random(-1.0, 1.0, &mut rng).unit_vector();
        }
        perlin_generate_noise(&mut perm_x);
        perlin_generate_noise(&mut perm_y);
        perlin_generate_noise(&mut perm_z);
        Perlin {
            seed: seed,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z,
            rndvec: rnd,
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        match 1 {
            1 => self.noise_perlin_interp(p),
            _ => self.noise_no_interp(p),
        }
    }

    pub fn noise_no_interp(&self, p: Vec3) -> f64 {
        let i = (((4.0 * p.x) as u32) & 255) as usize;
        let j = (((4.0 * p.y) as u32) & 255) as usize;
        let k = (((4.0 * p.z) as u32) & 255) as usize;
        let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.rndvec[idx as usize].x
    }

    pub fn noise_perlin_interp(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor().abs() as usize;
        let j = p.y.floor().abs() as usize;
        let k = p.z.floor().abs() as usize;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    c[di][dj][dk] = self.rndvec[idx as usize];
                }
            }
        }
        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Vec3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

fn perlin_generate_noise(p: &mut [u32; POINT_COUNT]) {
    for i in 0..POINT_COUNT {
        p[i] = i as u32;
    }
    permute(p);
}

fn permute(p: &mut [u32; POINT_COUNT]) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    for i in (0..POINT_COUNT).rev() {
        let target = rng.gen_range(0, POINT_COUNT + 1) as usize;
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let (ii, jj, kk) = (i as f64, j as f64, k as f64);
                let weight = vec3(u - ii, v - jj, w - kk);
                accum += (ii * uu + (1.0 - ii) * (1.0 - uu))
                    * (jj * vv + (1.0 - jj) * (1.0 - vv))
                    * (kk * ww + (1.0 - kk) * (1.0 - ww))
                    * c[i][j][k].dot(weight);
            }
        }
    }
    accum
}
