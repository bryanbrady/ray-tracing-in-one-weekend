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
use std::sync::mpsc::{channel,RecvError};

use rand::prelude::*;
use rand::rngs::SmallRng;

use camera::Camera;
use color::Color;
use color::color;
use color::write_color;
use hittable::{Hittable,HittableList};
use material::{MaterialScatter};
use material::Material;
use ray::Ray;
use shape::Shape;
use shape::sphere;
use sphere::Sphere;
use vec::Vec3;

extern crate threadpool;
use threadpool::ThreadPool;
// extern crate cpuprofiler;
// use cpuprofiler::PROFILER;

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
const SAMPLES_PER_PIXEL: u64 = 500;
const MAX_DEPTH: u32 = 50;
const GRID_SIZE: i32 = 5;

#[allow(dead_code)]
fn ray_color(ray : Ray, world: &HittableList, depth: u32, rng: &mut SmallRng) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0)
    }
    match world.hit(&ray, 0.0001, std::f64::INFINITY) {
        Some(hit) => {
            match hit.mat.scatter(&ray, &hit, rng) {
                Some(scatter) => {
                    return scatter.attenuation * ray_color(scatter.scattered, world, depth-1, rng);
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
    let material_ground = Material::lambertian(color(0.8, 0.8, 0.0));
    let material_center = Material::lambertian(color(0.1, 0.2, 0.5));
    let material_left   = Material::dielectric(1.5);
    let material_right  = Material::metal(color(0.8, 0.6, 0.2), 0.0);
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
    let material_left   = Material::lambertian(color(0.0, 0.0, 1.0));
    let material_right  = Material::lambertian(color(1.0, 0.0, 0.0));
    let sphere1 = Sphere { center: Vec3{x: -r, y: 0.0, z: -1.0},  radius: r, mat: material_left.clone()};
    let sphere2 = Sphere { center: Vec3{x:  r, y: 0.0, z: -1.0},  radius: r, mat: material_right.clone()};
    let mut world = HittableList::new();
    world.add(Shape::Sphere(sphere1));
    world.add(Shape::Sphere(sphere2));
    return world;
}

#[allow(dead_code)]
fn random_world() -> HittableList {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList::new();
    let material_ground = Material::lambertian(color(0.5, 0.5, 0.5));
    world.add(sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
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
                    let material = Material::lambertian(albedo);
                    world.add(sphere(center, 0.2, material));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Material::metal(albedo, fuzz);
                    world.add(sphere(center, 0.2, material));

                } else {
                    // glass
                    let material = Material::dielectric(1.5);
                    world.add(sphere(center, 0.2, material));
                }
            }
        }
    }

    world.add(sphere(Vec3::new(0.0, 1.0, 0.0),  1.0, Material::dielectric(1.5)));
    world.add(sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::lambertian(color(0.4, 0.2, 0.1))));
    world.add(sphere(Vec3::new(4.0, 1.0, 0.0),  1.0, Material::metal(color(0.7, 0.6, 0.5), 0.0)));
    return world;
}

#[allow(dead_code)]
fn random_world2() -> HittableList {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList::new();

    //let material_ground = Material::metal(color(0.0, 0.0, 0.0), 0.0);
    let material_ground = Material::lambertian(color(0.0, 0.0, 0.0));
    world.add(sphere(Vec3::new(0.0, -1100.0, 0.0), 1000.0, material_ground));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            for c in -GRID_SIZE..GRID_SIZE {
                let choose_mat: f64 = rng.gen::<f64>();
                let center = Vec3 {
                    x: (a as f64) + 1.5 * rng.gen::<f64>(),
                    y: (b as f64) + 1.5 * rng.gen::<f64>(),
                    z: (c as f64) + 1.5 * rng.gen::<f64>()
                };
                let some_point = Vec3::new(4.0, 0.2, 0.0);

                if (center - some_point).length() > 0.9 {
                    if choose_mat < 0.2 {
                        // diffuse
                        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                        let material = Material::lambertian(albedo);
                        world.add(sphere(center, 0.2, material));

                    } else if choose_mat < 0.75 {
                        // metal
                        let albedo = Color::random(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let material = Material::metal(albedo, fuzz);
                        world.add(sphere(center, 0.2, material));

                    } else {
                        // glass
                        let material = Material::dielectric(1.5);
                        world.add(sphere(center, 0.2, material));
                    }
                }
            }
        }
    }

    // world.add(sphere(Vec3::new(0.0, 1.0, 0.0),  1.0, Material::dielectric(1.5)));
    // world.add(sphere(Vec3::new(-4.0, 1.0, 0.0), 1.0, Material::lambertian(color(0.4, 0.2, 0.1))));
    // world.add(sphere(Vec3::new(4.0, 1.0, 0.0),  1.0, Material::metal(color(0.7, 0.6, 0.5), 0.0)));
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

#[allow(dead_code)]
fn camera_final() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 12.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus);
}

#[allow(dead_code)]
fn camera_other() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(1.0, 20.0, 1.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 18.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus);
}

fn main() -> Result<(), RecvError> {
    // PROFILER.lock().unwrap().start("./rt.profile").expect("Couldn't start");

    // Pixels
    let mut pixels = vec![color(0.0, 0.0, 0.0); PIXELS as usize];

    // World
    let world = random_world2();

    // Camera
    //let camera = camera_final();
    let camera = camera_other();

    // Parallelize
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    // Do it
    eprintln!("Tracing rays....");
    for h in 0..IMAGE_HEIGHT {
        let tx = tx.clone();
        let myworld = world.clone();
        pool.execute(move || for w in 0..IMAGE_WIDTH {
            let mut rng = SmallRng::from_entropy();
            let mut pixel_color = Color{r: 0.0, g: 0.0, b: 0.0};
            for _i in 0..SAMPLES_PER_PIXEL {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = ((w as f64) + ur) / ((IMAGE_WIDTH-1) as f64);
                let v: f64 = ((h as f64) + vr) / ((IMAGE_HEIGHT-1) as f64);
                let r = camera.get_ray(u, v, &mut rng);
                pixel_color += ray_color(r, &myworld, MAX_DEPTH, &mut rng);
            }
            tx.send((w,h,pixel_color)).expect("Could not send data!");
        });
    }

    for _ in 0..(IMAGE_HEIGHT * IMAGE_WIDTH) {
        let (w, h, pixel) = rx.recv()?;
        pixels[((IMAGE_HEIGHT-h-1)*IMAGE_WIDTH + w) as usize] = pixel;
    }

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels {
        write_color(&mut io::stdout(), pixel, SAMPLES_PER_PIXEL).expect("Unable to write data");
    }

    eprintln!("Done!\n");
    // PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    Ok(())
}
