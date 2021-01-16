mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod perlin;
mod ray;
mod scenes;
mod sphere;
mod texture;
mod util;
mod vec;

use std::io::{self};
use std::sync::mpsc::{channel, RecvError};

use rand::prelude::*;
use rand::rngs::SmallRng;

use bvh::BvhNode;
use color::{color, write_color, Color};
use hittable::{Hittable, Hittables};
use material::Material;
use ray::Ray;
use vec::vec3;

#[allow(unused_imports)]
use crate::scenes::{
    camera2, camera3, camera_blur, camera_final, camera_other, random_checkered_world,
    random_world, random_world2, random_world_original, world1, world2, perlin1
};

extern crate threadpool;
use threadpool::ThreadPool;
// extern crate cpuprofiler;
// use cpuprofiler::PROFILER;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
const SAMPLES_PER_PIXEL: u64 = 100;
const MAX_DEPTH: u32 = 50;
const GRID_SIZE: i32 = 11;

#[allow(dead_code)]
fn ray_color(ray: Ray, world: &Hittables, depth: u32, rng: &mut SmallRng) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }
    match world.hit(&ray, 0.0001, std::f64::INFINITY) {
        Some(hit) => match hit.mat.scatter(&ray, &hit, rng) {
            Some(scatter) => {
                return scatter.attenuation * ray_color(scatter.scattered, world, depth - 1, rng);
            }
            None => {
                return color(0.0, 0.0, 0.0);
            }
        },
        None => {
            let unit = ray.direction.unit_vector();
            let t = 0.5 * (unit.y + 1.0);
            let c = (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
            return color(c.x, c.y, c.z);
        }
    }
}

fn main() -> Result<(), RecvError> {
    // PROFILER.lock().unwrap().start("./rt.profile").expect("Couldn't start");

    // Pixels
    let mut pixels = vec![color(0.0, 0.0, 0.0); PIXELS as usize];

    // Time
    let (time0, time1) = (0.0, 0.0);

    // World
    //let world = Hittables::from(BvhNode::new(random_checkered_world(), time0, time1));
    //let world = Hittables::from(BvhNode::new(perlin1(), time0, time1));
    let world = Hittables::from(BvhNode::new(perlin1(), time0, time1));

    // Camera
    let camera = camera_final(time0, time1);

    // Parallelize
    let pool = ThreadPool::new(num_cpus::get() - 1);
    let (tx, rx) = channel();

    // Do it
    eprintln!("Tracing rays....");
    for h in 0..IMAGE_HEIGHT {
        let tx = tx.clone();
        let myworld = world.clone();
        pool.execute(move || {
            for w in 0..IMAGE_WIDTH {
                let mut rng = SmallRng::from_entropy();
                let mut pixel_color = Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                };
                for _i in 0..SAMPLES_PER_PIXEL {
                    let ur: f64 = rng.gen();
                    let vr: f64 = rng.gen();
                    let u: f64 = ((w as f64) + ur) / ((IMAGE_WIDTH - 1) as f64);
                    let v: f64 = ((h as f64) + vr) / ((IMAGE_HEIGHT - 1) as f64);
                    let r = camera.get_ray(u, v, &mut rng);
                    pixel_color += ray_color(r, &myworld, MAX_DEPTH, &mut rng);
                }
                tx.send((w, h, pixel_color)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(IMAGE_HEIGHT * IMAGE_WIDTH) {
        let (w, h, pixel) = rx.recv()?;
        pixels[((IMAGE_HEIGHT - h - 1) * IMAGE_WIDTH + w) as usize] = pixel;
    }

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels {
        write_color(&mut io::stdout(), pixel, SAMPLES_PER_PIXEL).expect("Unable to write data");
    }

    eprintln!("Done!\n");
    // PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    Ok(())
}
