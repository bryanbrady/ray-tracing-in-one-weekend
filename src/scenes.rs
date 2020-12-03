
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::{ASPECT_RATIO, GRID_SIZE};
use crate::camera::Camera;
use crate::color::{Color, color};
use crate::hittable::Hittables;
use crate::hittable_list::HittableList;
use crate::material::{Metal, Lambertian, Dielectric};
use crate::sphere::{Sphere,MovingSphere};
use crate::vec::Vec3;


#[derive(Debug)]
pub struct Scene{
    pub camera: Camera,
    pub hittables: Hittables,
}


#[allow(dead_code)]
pub fn world1() -> Hittables {
    // World 1
    let material_ground = Lambertian::new(color(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(color(0.1, 0.2, 0.5));
    let material_left   = Dielectric::new(1.5);
    let material_right  = Metal::new(color(0.8, 0.6, 0.2), 0.0);
    let sphere1 = Sphere::new(Vec3{x:  0.0,  y: -100.5, z: -1.0}, 100.0, material_ground.clone());
    let sphere2 = Sphere::new(Vec3{x:  0.0,  y:    0.0, z: -1.0},   0.5, material_center.clone());
    let sphere3 = Sphere::new(Vec3{x: -1.0,  y:    0.0, z: -1.0},   0.5, material_left.clone());
    let sphere4 = Sphere::new(Vec3{x: -1.0,  y:    0.0, z: -1.0}, -0.45, material_left.clone());
    let sphere5 = Sphere::new(Vec3{x:  1.0,  y:    0.0, z: -1.0},   0.5, material_right.clone());
    let mut world = HittableList{ hittables: Vec::new()};
    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);
    world.add(sphere5);
    return Hittables::from(world);
}

#[allow(dead_code)]
pub fn world2() -> Hittables {
    // World 2
    let r = f64::cos(std::f64::consts::PI / 4.0);
    let material_left   = Lambertian::new(color(0.0, 0.0, 1.0));
    let material_right  = Lambertian::new(color(1.0, 0.0, 0.0));
    let sphere1 = Sphere::new(Vec3{x: -r, y: 0.0, z: -1.0},  r, material_left.clone());
    let sphere2 = Sphere::new(Vec3{x:  r, y: 0.0, z: -1.0},  r, material_right.clone());
    let mut world = HittableList{ hittables: Vec::new()};
    world.add(sphere1);
    world.add(sphere2);
    return Hittables::from(world);
}

#[allow(dead_code)]
pub fn random_world_original() -> Hittables {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList{ hittables: Vec::new()};
    let material_ground = Lambertian::new(color(0.5, 0.5, 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

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
                    let albedo = Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, material));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, material));

                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0),  1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(color(0.4, 0.2, 0.1))));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0),  1.0, Metal::new(color(0.7, 0.6, 0.5), 0.0)));
    return Hittables::from(world);
}

#[allow(dead_code)]
pub fn random_world() -> Hittables {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList{ hittables: Vec::new()};
    let material_ground = Lambertian::new(color(0.5, 0.5, 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

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
                    let albedo = Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(albedo);
                    let center2 = center +  Vec3::new(0.0, rng.gen_range(0.0, 0.25), 0.0);
                    world.add(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, material));

                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0),  1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Lambertian::new(color(0.4, 0.2, 0.1))));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0),  1.0, Metal::new(color(0.7, 0.6, 0.5), 0.0)));
    return Hittables::from(world);
}

#[allow(dead_code)]
pub fn random_world2() -> Hittables {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList{ hittables: Vec::new()};

    let material_ground = Lambertian::new(color(0.0, 0.0, 0.0));
    world.add(Sphere::new(Vec3::new(0.0, -1100.0, 0.0), 1000.0, material_ground));

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
                        let albedo = Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                        let material = Lambertian::new(albedo);
                        world.add(Sphere::new(center, 0.2, material));

                    } else if choose_mat < 0.75 {
                        // metal
                        let albedo = Color::random(0.5, 1.0, &mut rng);
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let material = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, 0.2, material));

                    } else {
                        // glass
                        let material = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, material));
                    }
                }
            }
        }
    }

    return Hittables::from(world);
}

#[allow(dead_code)]
pub fn camera2() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat= Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 2.0;
    let dist_to_focus = (lookfrom-lookat).length();
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
}

#[allow(dead_code)]
pub fn camera3() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
}

#[allow(dead_code)]
pub fn camera_final() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 12.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
}

#[allow(dead_code)]
pub fn camera_blur() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);
}

#[allow(dead_code)]
pub fn camera_other() -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = Vec3::new(1.0, 20.0, 1.0);
    let lookat= Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 18.0;
    return Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
}

