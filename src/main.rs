
use std::io::{self};

mod color;
mod vec;

fn main() -> io::Result<()> {
    let image_width = 200;
    let image_height = 200;
    for h in  (0..image_height).rev() {
        // eprintln!("Scanlines remaining: {}", h);
        for w in 0..image_width {
            let color = color::Color {
                r: (w as f64) / (image_width as f64),
                g: (h as f64) / (image_height as f64),
                b: 0.2
            };
            color::write_color(&mut io::stdout(), color)?;
        }
    }
    // eprintln!("\nDone!\n");
    Ok(())
}
