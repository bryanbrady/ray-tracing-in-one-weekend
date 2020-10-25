mod camera;
mod color;
mod hittable;
mod material;
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
use material::{Dielectric,Lambertian,Metal};
use ray::Ray;
use shape::Shape;
use shape::sphere;
use sphere::Sphere;
use vec::Vec3;

use std::rc::Rc;

//const ASPECT_RATIO: f64 = 16.0 / 9.0;
const ASPECT_RATIO: f64 = 3.0 / 2.0;

#[allow(dead_code)]
fn ray_color(ray : Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0)
    }
    match world.hit(&ray, 0.0001, std::f64::INFINITY) {
        Some(hit) => {
            match hit.mat.scatter(&ray, &hit) {
                Some(scatter) => {
                    return scatter.attenuation * ray_color(scatter.scattered, world, depth-1);
                }
                None => {
                    return color(0.0, 0.0, 0.0);
                }

            }
        }
        None => {
            let unit = ray.direction.unit_vector();
            let t = 0.5 * (unit.y + 1.0);
            let c = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0)  + t * Vec3::new(0.5, 0.7, 1.0);
            return color(c.x, c.y, c.z);
        }

    }
}

#[allow(dead_code)]
fn world1() -> HittableList {
    // World 1
    let material_ground = Rc::new(Lambertian { albedo: color(0.8, 0.8, 0.0) });
    let material_center = Rc::new(Lambertian { albedo: color(0.1, 0.2, 0.5) });
    let material_left   = Rc::new(Dielectric { ir: 1.5 });
    let material_right  = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 0.0));
    let sphere1 = Sphere { center: Vec3{x:  0.0,  y: -100.5, z: -1.0}, radius: 100.0, mat: material_ground.clone()};
    let sphere2 = Sphere { center: Vec3{x:  0.0,  y:    0.0, z: -1.0}, radius:   0.5, mat: material_center.clone()};
    let sphere3 = Sphere { center: Vec3{x: -1.0,  y:    0.0, z: -1.0}, radius:   0.5, mat: material_left.clone()};
    let sphere4 = Sphere { center: Vec3{x: -1.0,  y:    0.0, z: -1.0}, radius: -0.45, mat: material_left.clone()};
    let sphere5 = Sphere { center: Vec3{x:  1.0,  y:    0.0, z: -1.0}, radius:   0.5, mat: material_right.clone()};
    let mut world = HittableList::new();
    world.add(Shape::Sphere(sphere1));
    world.add(Shape::Sphere(sphere2));
    world.add(Shape::Sphere(sphere3));
    world.add(Shape::Sphere(sphere4));
    world.add(Shape::Sphere(sphere5));
    return world;
}

#[allow(dead_code)]
fn world2() -> HittableList {
    // World 2
    let r = f64::cos(std::f64::consts::PI / 4.0);
    let material_left   = Rc::new(Lambertian {albedo: color(0.0, 0.0, 1.0)});
    let material_right  = Rc::new(Lambertian {albedo: color(1.0, 0.0, 0.0)});
    let sphere1 = Sphere { center: Vec3{x: -r, y: 0.0, z: -1.0},  radius: r, mat: material_left.clone()};
    let sphere2 = Sphere { center: Vec3{x:  r, y: 0.0, z: -1.0},  radius: r, mat: material_right.clone()};
    let mut world = HittableList::new();
    world.add(Shape::Sphere(sphere1));
    world.add(Shape::Sphere(sphere2));
    return world;
}

fn random_world() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian{albedo: color(0.5, 0.5, 0.5)});
    world.add(sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>()
            };
            let some_point = Vec3::new(4.0, 0.2, 0.0);

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let material = Rc::new(Lambertian{albedo: albedo});
                    world.add(sphere(center, 0.2, material));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Rc::new(Metal{albedo: albedo, fuzz: fuzz});
                    world.add(sphere(center, 0.2, material));

                } else {
                    // glass
                    let material = Rc::new(Dielectric{ir: 1.5});
                    world.add(sphere(center, 0.2, material));
                }

            }
        }
    }

    world.add(sphere(Vec3::new(0.0, 1.0, 0.0),  1.0, Rc::new(Dielectric{ir: 1.5})));
    world.add(sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian{albedo: color(0.4, 0.2, 0.1)})));
    world.add(sphere(Vec3::new(4.0, 1.0, 0.0),  1.0, Rc::new(Metal{albedo: color(0.7, 0.6, 0.5), fuzz: 0.0})));
    return world;
}

#[allow(dead_code)]
fn camera2() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat= Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 2.0;
    let dist_to_focus = (lookfrom-lookat).length();
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus);
}

#[allow(dead_code)]
fn camera3() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus);
}

fn camera_final() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus);
}

fn main() -> io::Result<()> {

    // Image
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = random_world();

    // Camera
    let camera = camera_final();

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
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!("Done!\n");
    Ok(())
}
