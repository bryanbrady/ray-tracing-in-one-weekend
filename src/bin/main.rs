#[cfg(feature = "profile")]
use cpuprofiler::PROFILER;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::io::{self};
use std::sync::Arc;
use structopt::StructOpt;

use rtlib::color::{color, write_color, Color};
use rtlib::hittable::{Hittable, Hittables};
use rtlib::material::Material;
use rtlib::pdf::{HittablePdf, MixturePdf, Pdf};
use rtlib::ray::Ray;

#[allow(unused_imports)]
use rtlib::scenes::{
    cornell_box::cornell_box,
    cornell_box::cornell_box_sphere,
    cornell_smoke::cornell_smoke,
    next_week_final::next_week_final,
    perlin::marble,
    perlin::noise,
    perlin::turbulence,
    random_world::random_world,
    random_world::random_world_checkered,
    random_world::random_world_earth,
    random_world::random_world_original,
    rotate_test::rotate_test,
    simple_light::simple_light,
};

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
                    let pdf = scatter.pdf;
                    if pdf.is_none() || lights.length() == 0 {
                        return scatter.attenuation
                            * ray_color(scatter.ray, background, world, lights.clone(), depth - 1, rng);
                    } else {
                        let pdf = pdf.unwrap();
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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, default_value = "400")]
    width: u64,

    #[structopt(short, long, default_value = "100")]
    samples: u64,

    #[structopt(short, long, default_value = "1.0")]
    aspect_ratio: f64,

    #[structopt(short, long, default_value = "50")]
    depth: u32,

    #[structopt(short, long, default_value = "random_world")]
    scene: String,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    let aspect_ratio = opt.aspect_ratio;
    let image_width = opt.width;
    let image_height = ((image_width as f64) / aspect_ratio) as u64;
    let num_pixels = image_width * image_height;
    let samples_per_pixel = opt.samples;
    let max_depth = opt.depth;

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() - 1)
        .build_global()
        .unwrap();

    #[cfg(feature = "profile")]
    {
        PROFILER
            .lock()
            .unwrap()
            .start("./rt.profile")
            .expect("Couldn't start");
    }

    // Time
    let (time0, time1) = (0.0, 1.0);

    // Scene
    let scene = match opt.scene.as_ref() {
        "cornell_box" => cornell_box(time0, time1, aspect_ratio),
        "cornell_box_sphere" => cornell_box_sphere(time0, time1, aspect_ratio),
        "cornell_smoke" => cornell_smoke(time0, time1, aspect_ratio),
        "next_week_final" => next_week_final(time0, time1, aspect_ratio),
        "marble" => marble(time0, time1, aspect_ratio),
        "noise" => noise(time0, time1, aspect_ratio),
        "turbulence" => turbulence(time0, time1, aspect_ratio),
        "random_world" => random_world(time0, time1, aspect_ratio),
        "random_world_checkered" => random_world_checkered(time0, time1, aspect_ratio),
        "random_world_earth" => random_world_earth(time0, time1, aspect_ratio),
        "random_world_original" => random_world_original(time0, time1, aspect_ratio),
        "rotate_test" => rotate_test(time0, time1, aspect_ratio),
        "simple_light" => simple_light(time0, time1, aspect_ratio),
        _ => random_world(time0, time1, aspect_ratio),
    };

    // World
    let world = scene.hittables;
    let lights = Arc::new(scene.lights);

    // Camera
    let camera = scene.camera;
    let background = camera.background;

    // Progress Bar
    let bar = ProgressBar::new(num_pixels);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {msg}"),
    );

    // Do it
    eprintln!("Tracing rays....");
    let pixels: Vec<Color> = (0..num_pixels)
        .into_par_iter()
        .progress_with(bar)
        .map_init(SmallRng::from_entropy, |rng, i| {
            let h = image_height - (i / image_width) - 1;
            let w = i % image_width;
            let mut pixel_color = color(0.0, 0.0, 0.0);
            for _i in 0..samples_per_pixel {
                let ur: f64 = rng.gen();
                let vr: f64 = rng.gen();
                let u: f64 = ((w as f64) + ur) / ((image_width - 1) as f64);
                let v: f64 = ((h as f64) + vr) / ((image_height - 1) as f64);
                let r = camera.get_ray(u, v, rng);
                pixel_color += ray_color(r, background, &world, lights.clone(), max_depth, rng);
            }
            pixel_color
        })
        .collect();

    println!("P3\n{} {}\n255", image_width, image_height);
    for pixel in pixels {
        write_color(&mut io::stdout(), pixel, samples_per_pixel).expect("Unable to write data");
    }

    eprintln!("Done!\n");
    #[cfg(feature = "profile")]
    {
        PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    }
    Ok(())
}
