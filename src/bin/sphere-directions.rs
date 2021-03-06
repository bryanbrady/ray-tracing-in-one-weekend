use rand::prelude::*;
use rand::rngs::SmallRng;
use rtlib::vec::Vec3;

fn pdf(_p: Vec3) -> f64 {
    1.0 / (4.0 * std::f64::consts::PI)
}

const N: u64 = 1000000;
const SEED: u64 = 0;

fn main() -> Result<(), std::io::Error> {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut sum = 0.0;
    for _ in 0..N {
        let d = Vec3::random_unit_vector(&mut rng);
        let cosine_squared = d.z * d.z;
        sum += cosine_squared / pdf(d);
    }
    println!("I = {:.12} ({} iterations)", sum / (N as f64), N);

    Ok(())
}
