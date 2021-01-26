use rtlib::util::random_double;
use rand::prelude::*;
use rand::rngs::SmallRng;

const N : u64 = 200;
const SEED : u64 = 0;

fn main() -> Result<(), std::io::Error> {
    let mut rng = SmallRng::seed_from_u64(SEED);
    for _ in 0..N {
        let r1 = random_double(&mut rng);
        let r2 = random_double(&mut rng);
        let x = f64::cos(2.0 * std::f64::consts::PI * r1) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
        let y = f64::sin(2.0 * std::f64::consts::PI * r1) * 2.0 * f64::sqrt(r2 * (1.0 - r2));
        let z = 1.0 - 2.0 * r2;
        println!("{} {} {}", x, y, z);
    }

    Ok(())
}
