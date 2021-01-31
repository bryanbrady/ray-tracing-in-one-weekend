use rand::prelude::*;
use rand::rngs::SmallRng;
use rtlib::util::random_double;

const N: u64 = 1000000;
const SEED: u64 = 0;

fn main() -> Result<(), std::io::Error> {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut sum = 0.0;
    for _ in 0..N {
        // let r1 = random_double(&mut rng);
        let r2 = random_double(&mut rng);
        // let x = f64::cos(2.0 * std::f64::consts::PI * r1) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
        // let y = f64::sin(2.0 * std::f64::consts::PI * r1) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
        let z = 1.0 - r2;
        sum += z * z * z / (1.0 / (2.0 * std::f64::consts::PI));
    }
    println!("PI/2     = {}", std::f64::consts::PI / 2.0);
    println!("Estimate = {}", sum / (N as f64));

    Ok(())
}
