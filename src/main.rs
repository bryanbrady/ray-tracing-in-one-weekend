mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod scenes;
mod texture;
mod util;
mod vec;

use std::io::{self};
use std::sync::mpsc::{channel, RecvError};

use rand::prelude::*;
use rand::rngs::SmallRng;

use color::{color, write_color, Color};
use hittable::{bvh::BvhNode, Hittable, Hittables};
use material::Material;
use ray::Ray;

#[allow(unused_imports)]
use crate::scenes::{
    camera_next_week_final, camera2, camera3, camera_blur, camera_cornell_box, camera_final, camera_light, camera_other,
    cornell_box, cornell_smoke, earth, marble1, noise1, random_checkered_world, random_world, random_world2,
    random_world_earth, random_world_original, rotate_test, simple_light, turbulence1, world1,
    world2, next_week_final
};

extern crate threadpool;
use threadpool::ThreadPool;
extern crate cpuprofiler;
use cpuprofiler::PROFILER;

// Image
//const ASPECT_RATIO: f64 = 16.0 / 9.0;
const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
const PIXELS: u32 = IMAGE_WIDTH * IMAGE_HEIGHT;
const SAMPLES_PER_PIXEL: u64 = 100;
const MAX_DEPTH: u32 = 50;
const GRID_SIZE: i32 = 11;

#[allow(dead_code)]
fn ray_color(
    ray: Ray,
    background: Color,
    world: &Hittables,
    depth: u32,
    rng: &mut SmallRng,
) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }
    match world.hit(&ray, 0.0001, std::f64::MAX, rng) {
        Some(hit) => {
            let emitted = hit.mat.emitted(hit.u, hit.v, hit.point);
            match hit.mat.scatter(&ray, &hit, rng) {
                Some(scatter) => {
                    return emitted
                        + scatter.attenuation
                            * ray_color(scatter.scattered, background, world, depth - 1, rng);
                }
                None => {
                    return emitted;
                }
            }
        }
        None => {
            return background;
        }
    }
}

fn main() -> Result<(), RecvError> {
    PROFILER.lock().unwrap().start("./rt.profile").expect("Couldn't start");

    // Pixels
    let mut pixels = vec![color(0.0, 0.0, 0.0); PIXELS as usize];

    // Time
    let (time0, time1) = (0.0, 1.0);

    // World
    // let world = Hittables::from(BvhNode::new(random_world_earth(), time0, time1));
    // let world = Hittables::from(BvhNode::new(marble1(), time0, time1));
    // let world = Hittables::from(BvhNode::new(simple_light(), time0, time1));
    // let world = Hittables::from(BvhNode::new(rotate_test(), time0, time1));
    // let world = Hittables::from(BvhNode::new(cornell_box(), time0, time1));
    // let world = Hittables::from(BvhNode::new(cornell_smoke(), time0, time1));
    let world = Hittables::from(BvhNode::new(next_week_final(), time0, time1));

    // Camera
    // let camera = camera_cornell_box(time0, time1);
    // let camera = camera_final(time0, time1);
    // let camera = camera_light(time0, time1);
    let camera = camera_next_week_final(time0, time1);
    let background = camera.background;

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
                    pixel_color += ray_color(r, background, &myworld, MAX_DEPTH, &mut rng);
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
    PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    Ok(())
}
