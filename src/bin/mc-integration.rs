use rand::Rng;

fn pdf1(x: f64) -> f64 {
    0.5 * x
}

fn pdf2(_: f64) -> f64 {
    0.5
}

fn pdf3(x: f64) -> f64 {
    3.0 * x * x / 8.0
}

const N : u64 = 1000000;

fn one() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in 0..N {
        let x = rng.gen_range(0.0, 2.0);
        sum += x*x;
    }
    println!("I = {:.12} ({} iterations)", 2.0 * sum / (N as f64), N);

    Ok(())
}

fn two() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in 0..N {
        let x = f64::sqrt(rng.gen_range(0.0, 4.0));
        sum += x*x / pdf1(x);
    }
    println!("I = {:.12} ({} iterations)", sum / (N as f64), N);

    Ok(())
}

fn three() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in 0..N {
        let x = rng.gen_range(0.0, 2.0);
        sum += x*x / pdf2(x);
    }
    println!("I = {:.12} ({} iterations)", sum / (N as f64), N);

    Ok(())
}

fn four() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut sum = 0.0;
    for _ in 0..N {
        let x = f64::powf(rng.gen_range(0.0, 2.0), 1.0/3.0);
        sum += x*x / pdf3(x);
    }
    println!("I = {:.12} ({} iterations)", sum / (N as f64), N);

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    println!("Integrating x^2");
    one()?;
    println!("Integrating x^2 with PDF");
    two()?;
    println!("Integrating x^2, v3");
    three()?;
    println!("Integrating x^2, v3");
    four()?;
    Ok(())
}
