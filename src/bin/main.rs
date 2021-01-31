#[cfg(feature = "profile")]
use cpuprofiler::PROFILER;

use indicatif::{ProgressBar, ProgressStyle, ParallelProgressIterator};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::prelude::*;
use rayon::iter::{ParallelIterator};
use std::io::{self};
use std::sync::Arc;

use rtlib::color::{color, write_color, Color};
use rtlib::hittable::{Hittable, Hittables};
use rtlib::hittable::hittable_list::HittableList;
use rtlib::material::{Material, MaterialType};
use rtlib::pdf::{HittablePdf, MixturePdf, Pdf};
use rtlib::ray::Ray;

#[allow(unused_imports)]
use rtlib::scenes::{
    cornell_box::cornell_box,
    cornell_box::cornell_box_test,
    cornell_smoke::cornell_smoke,
    next_week_final::next_week_final,
    perlin::noise,
    perlin::marble,
    perlin::turbulence,
    random_world::random_world,
    random_world::random_world_original,
    random_world::random_world_checkered,
    random_world::random_world_earth,
    rotate_test::rotate_test,
    simple_light::simple_light,
};

use rtlib::hittable::rect::XzRect;

// Image
const ASPECT_RATIO: f64 = 1.0;
// const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 800;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
const PIXELS: u64 = IMAGE_WIDTH * IMAGE_HEIGHT;
const SAMPLES_PER_PIXEL: u64 = 2000;
const MAX_DEPTH: u32 = 50;

#[allow(dead_code)]
fn ray_color(
    ray: Ray,
    background: Color,
    world: &Hittables,
    lights: Arc<Hittables>,
    depth: u32,
    rng: &mut SmallRng,
) -> Color {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }
    match world.hit(&ray, 0.0001, std::f64::MAX, rng) {
        Some(hit) => {
            let emitted = hit.mat.emitted(&ray, &hit, hit.u, hit.v, hit.point);
            match hit.mat.scatter(&ray, &hit, rng) {
                Some(scatter) => {
                    match scatter.pdf {
                        None => {
                            return scatter.attenuation
                                * ray_color(scatter.ray, background, world, lights.clone(), depth - 1, rng)
                        },
                        Some(pdf) => {
                            let light_pdf = HittablePdf::new(hit.point, lights.clone());
                            let pdf = MixturePdf::new(Arc::new(light_pdf), Arc::new(pdf));
                            let scattered = Ray {
                                origin: hit.point,
                                direction: pdf.generate(rng),
                                time: ray.time,
                            };
                            let pdf_val = pdf.value(scattered.direction, rng);
                            return emitted
                                + scatter.attenuation
                                    * hit.mat.scattering_pdf(&ray, &hit, &scattered)
                                    * ray_color(scattered, background, world, lights.clone(), depth - 1, rng)
                                    * (1.0 / pdf_val);
                        }
                    }
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

fn main() -> Result<(), std::io::Error> {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get()-1).build_global().unwrap();

    #[cfg(feature = "profile")]
    {
        PROFILER.lock().unwrap().start("./rt.profile").expect("Couldn't start");
    }

    // Time
    let (time0, time1) = (0.0, 1.0);

    // Scene
    //let scene = cornell_box(time0, time1, ASPECT_RATIO);
    // let scene = cornell_box_test(time0, time1, ASPECT_RATIO);
    // let lights = Arc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, Arc::new(MaterialType::default())));
    let scene = random_world_original(time0, time1, ASPECT_RATIO);

    // World
    let world = scene.hittables;
    let mut lights = HittableList {
        hittables: Vec::new(),
    };
    lights.add(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, Arc::new(MaterialType::default())));
    //lights.add(Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, Arc::new(MaterialType::default())));
    let lights = Arc::new(Hittables::from(lights));

    // Camera
    let camera = scene.camera;
    let background = camera.background;

    // Progress Bar
    let bar = ProgressBar::new(PIXELS);
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {msg}"));

    // Do it
    eprintln!("Tracing rays....");
    let pixels: Vec<Color>  = (0..PIXELS)
        .into_par_iter()
        .progress_with(bar)
        .map_init(
            SmallRng::from_entropy,
            |rng, i| {
                let h = IMAGE_HEIGHT - (i / IMAGE_WIDTH) - 1;
                let w = i % IMAGE_WIDTH;
                let mut pixel_color = color(0.0, 0.0, 0.0);
                for _i in 0..SAMPLES_PER_PIXEL {
                    let ur: f64 = rng.gen();
                    let vr: f64 = rng.gen();
                    let u: f64 = ((w as f64) + ur) / ((IMAGE_WIDTH - 1) as f64);
                    let v: f64 = ((h as f64) + vr) / ((IMAGE_HEIGHT - 1) as f64);
                    let r = camera.get_ray(u, v, rng);
                    pixel_color += ray_color(r, background, &world, lights.clone(), MAX_DEPTH, rng);
                }
                pixel_color
            },
        ).collect();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for pixel in pixels {
        write_color(&mut io::stdout(), pixel, SAMPLES_PER_PIXEL).expect("Unable to write data");
    }

    eprintln!("Done!\n");
    #[cfg(feature = "profile")]
    {
        PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    }
    Ok(())
}
