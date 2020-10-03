
use std::io::{self};

fn main() -> io::Result<()> {
    let image_width = 200;
    let image_height = 200;
    println!("P3\n{} {}\n255",image_width, image_height);
    for h in  (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", h);
        for w in 0..image_width {
            let r = (w as f64) / (image_width as f64);
            let g = (h as f64) / (image_height as f64);
            let b = 0.2;
            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone!\n");
    Ok(())
}
