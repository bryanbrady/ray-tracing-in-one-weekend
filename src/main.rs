
use std::io::{self};

mod color;
mod vec;

fn main() -> io::Result<()> {
    let image_width = 200;
    let image_height = 200;
    println!("P3\n{} {}\n255",image_width, image_height);
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
    let a1 = vec::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let a2 = vec::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let aa = a1 + a2;
    eprintln!("{:?}", aa);
    eprintln!("{:?}", aa.x);
    eprintln!("{:?}", aa.y);
    eprintln!("{:?}", aa.z);
    let s1 = vec::Vec3 { x: 10.0, y: 20.0, z: 30.0 };
    let s2 = vec::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let ss = s1 - s2;
    eprintln!("{:?}", ss);
    eprintln!("{:?}", ss.x);
    eprintln!("{:?}", ss.y);
    eprintln!("{:?}", ss.z);
    let b1 = vec::Vec3 { x: 10.0, y: 20.0, z: 30.0 };
    let b2 = 2.0;
    let bb = b1 / b2;
    eprintln!("{:?}", bb);
    eprintln!("{:?}", bb.x);
    eprintln!("{:?}", bb.y);
    eprintln!("{:?}", bb.z);
    let u1 = vec::Vec3 {  x: 1.0, y: 1.0, z: 1.0 };
    let u2 = vec::Vec3 {  x: 1.0, y: 0.0, z: 0.0 };
    eprintln!("{:?}", u1.length());
    eprintln!("{:?}", u2.length());


    // eprintln!("\nDone!\n");
    Ok(())
}
