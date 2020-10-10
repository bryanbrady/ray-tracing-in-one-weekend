
use std::io::{self};

mod color;
use color::Color;
use color::write_color;

mod ray;
use ray::Ray;

mod vec;
use vec::Vec3;


#[allow(dead_code)]
fn ray_color(_ray : ray::Ray) -> color::Color {
    let unit = _ray.direction.unit_vector();
    let t = 0.5 * (unit.y + 1.0);
    let c = Vec3{x: 1.0, y: 1.0, z: 1.0} * (1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0} * t;
    Color{ r: c.x, g: c.y, b: c.z }
}


fn main() -> io::Result<()> {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIG: Vec3 = Vec3{x: 0.0, y: 0.0, z: 0.0};
    const HORIZONTAL: Vec3  = Vec3{x: VIEWPORT_WIDTH, y: 0.0, z: 0.0};
    const VERTICAL: Vec3  = Vec3{x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0};
    let lower_left = ORIG - HORIZONTAL/2.0
                   - VERTICAL/2.0
                   - Vec3{x: 0.0, y: 0.0, z: FOCAL_LENGTH};

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for h in  (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", h);
        for w in 0..IMAGE_WIDTH {
            let u = (w as f64) / ((IMAGE_WIDTH-1) as f64);
            let v = (h as f64) / ((IMAGE_HEIGHT-1) as f64);
            let r = Ray{
                origin: ORIG,
                direction: &lower_left + HORIZONTAL*u + VERTICAL*v - ORIG
            };
            write_color(&mut io::stdout(), ray_color(r))?;
        }
    }
    eprintln!("Done!\n");
    Ok(())
}
