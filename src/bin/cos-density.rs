use rand::prelude::*;
use rand::rngs::SmallRng;
use rtlib::util::random_double;
use rtlib::vec::{vec3, Vec3};

const N: u64 = 1000000;
const SEED: u64 = 0;

fn random_cosine_direction(rng: &mut SmallRng) -> Vec3 {
    let r1 = random_double(rng);
    let r2 = random_double(rng);
    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = f64::cos(phi) * f64::sqrt(r2);
    let y = f64::sin(phi) * f64::sqrt(r2);
    let z = f64::sqrt(1.0 - r2);
    vec3(x, y, z)
}

fn main() -> Result<(), std::io::Error> {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut sum = 0.0;
    for _ in 0..N {
        let v = random_cosine_direction(&mut rng);
        sum += v.z * v.z * v.z / (v.z / std::f64::consts::PI);
    }
    println!("PI/2     = {}", std::f64::consts::PI / 2.0);
    println!("Estimate = {}", sum / (N as f64));

    Ok(())
}
