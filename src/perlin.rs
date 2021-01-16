use std::fmt;

use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::vec::Vec3;

const SEED: u64 = 0;
const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    pub seed: u64,
    pub perm_x: [u32; POINT_COUNT],
    pub perm_y: [u32; POINT_COUNT],
    pub perm_z: [u32; POINT_COUNT],
    pub rndf64: [f64; POINT_COUNT],
}

impl fmt::Debug for Perlin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Perlin")
         .field("seed", &self.seed)
         .finish()
    }
}

impl Perlin{

    pub fn new(seed: u64) -> Perlin {
        let mut rng = SmallRng::seed_from_u64(SEED);
        let mut rnd: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        let mut perm_x: [u32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_y: [u32; POINT_COUNT] = [0; POINT_COUNT];
        let mut perm_z: [u32; POINT_COUNT] = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            rnd[i] = rng.gen_range(0.0, 1.0);
        }
        perlin_generate_noise(&mut perm_x);
        perlin_generate_noise(&mut perm_y);
        perlin_generate_noise(&mut perm_z);
        Perlin {
            seed: seed,
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z,
            rndf64: rnd
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (((4.0 * p.x) as u32) & 255) as usize;
        let j = (((4.0 * p.y) as u32) & 255) as usize;
        let k = (((4.0 * p.z) as u32) & 255) as usize;
        let idx = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.rndf64[idx as usize]
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
