
use rand::Rng;

fn main() -> Result<(), std::io::Error> {
    let sqrt_n: u64 = 10000;
    let mut rng = rand::thread_rng();
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;
    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = rng.gen_range(-1.0, 1.0);
            let y = rng.gen_range(-1.0, 1.0);
            if x*x + y*y < 1.0 {
                inside_circle += 1;
            }
            let x = 2.0 * ((i as f64) + rng.gen_range(0.0, 1.0)) / (sqrt_n as f64) - 1.0;
            let y = 2.0 * ((j as f64) + rng.gen_range(0.0, 1.0)) / (sqrt_n as f64) - 1.0;
            if x*x + y*y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }
    let n = sqrt_n * sqrt_n;
    println!("Estimate of Pi: {:.12} ({} iterations)", 4.0*(inside_circle as f64) / (n as f64), n);
    println!("Estimate of Pi: {:.12} ({} iterations)", 4.0*(inside_circle_stratified as f64) / (n as f64), n);

    Ok(())
}
