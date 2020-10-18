mod camera;
mod color;
mod hittable;
mod ray;
mod shape;
mod sphere;
mod vec;
mod util;

use std::io::{self};
use rand::prelude::*;

use camera::Camera;
use color::Color;
use color::color;
use color::write_color;
use hittable::{Hittable,HittableList};
use ray::Ray;
use shape::Shape;
use sphere::Sphere;
use vec::Vec3;


#[allow(dead_code)]
fn ray_color(ray : Ray, world: &HittableList) -> Color {
    match world.hit(&ray, 0.0, std::f64::INFINITY) {
        Some(hit) => {
            let n = 0.5 * (hit.normal.unit_vector() + 1.0);
            let c = color(n.x, n.y, n.z);
            return c
        }
        None => {
            let unit = ray.direction.unit_vector();
            let t = 0.5 * (unit.y + 1.0);
            let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0)  + t * Vec3::new(0.5, 0.7, 1.0);
            color(c.x, c.y, c.z)
        }

    }
}

fn main() -> io::Result<()> {

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;


    // World
    let sphere1 = Sphere { center: Vec3{x: 0.0,  y: 0.0, z: -1.0}, radius: 0.5 };
    let sphere2 = Sphere { center: Vec3{x: 0.0,  y: -100.5, z: -1.0}, radius: 100.0 };
    let mut world = HittableList::new();
    world.add(Shape::Sphere(sphere1));
    world.add(Shape::Sphere(sphere2));

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, FOCAL_LENGTH);

    let mut rng = rand::thread_rng();
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for h in  (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", h);
        for w in 0..IMAGE_WIDTH {
            let mut pixel_color = Color{r: 0.0, g: 0.0, b: 0.0};
            for _i in 0..SAMPLES_PER_PIXEL {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = ((w as f64) + ur) / ((IMAGE_WIDTH-1) as f64);
                let v: f64 = ((h as f64) + vr) / ((IMAGE_HEIGHT-1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }

            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!("Done!\n");
    Ok(())
}
