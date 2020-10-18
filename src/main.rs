mod color;
mod hittable;
mod ray;
mod shape;
mod sphere;
mod vec;

use std::io::{self};

use color::Color;
use color::color;
use color::write_color;

use hittable::{Hittable,HittableList};
use ray::Ray;
use shape::Shape;
use sphere::Sphere;
use vec::Vec3;


#[allow(dead_code)]
fn hit_sphere(center: Vec3, radius: f64, r : Ray) -> f64{
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - f64::sqrt(discriminant)) / a
    }
}

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
            write_color(&mut io::stdout(), ray_color(r, &world))?;
        }
    }
    eprintln!("Done!\n");
    Ok(())
}
