use crate::camera::{Camera, CameraConfig};
use crate::color::{color, Color};
use crate::hittable::{
    bvh::BvhNode,
    hittable_list::HittableList,
    sphere::{MovingSphere, Sphere},
    Hittables,
};
use crate::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::scenes::Scene;
use crate::texture::{checker::CheckerTexture, image::ImageTexture, solidcolor::SolidColor};
use crate::vec::{vec3, Vec3};

use rand::prelude::*;
use rand::rngs::SmallRng;

const GRID_SIZE: i32 = 11;

#[allow(dead_code)]
pub fn random_world_original(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 12.0,
        time0: t0,
        time1: t1,
        background: color(0.7, 0.8, 1.0),
    });
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    let material_ground = Lambertian::new(SolidColor::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>(),
            };
            let some_point = vec3(4.0, 0.2, 0.0);

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(SolidColor::new(albedo.r, albedo.g, albedo.b));
                    world.add(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(SolidColor::new(albedo.r, albedo.g, albedo.b), fuzz);
                    world.add(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1));
    world.add(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, mat2));
    world.add(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, mat3));
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}

#[allow(dead_code)]
pub fn random_world_checkered(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 12.0,
        time0: t0,
        time1: t1,
        background: color(0.7, 0.8, 1.0),
    });
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    let texture_ground = CheckerTexture::new(color(0.9, 0.9, 0.9), color(0.1, 0.6, 0.1));
    let material_ground = Lambertian::new(texture_ground);
    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>(),
            };
            let some_point = vec3(4.0, 0.2, 0.0);

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(SolidColor::new(albedo.r, albedo.g, albedo.b));
                    let center2 = center + vec3(0.0, rng.gen_range(0.0, 0.25), 0.0);
                    world.add(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(SolidColor::new(albedo.r, albedo.g, albedo.b), fuzz);
                    world.add(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1));
    world.add(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, mat2));
    world.add(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, mat3));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}

#[allow(dead_code)]
pub fn random_world(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 12.0,
        time0: t0,
        time1: t1,
        background: color(0.7, 0.8, 1.0),
    });
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    let material_ground = Lambertian::new(SolidColor::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>(),
            };
            let some_point = vec3(4.0, 0.2, 0.0);

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(SolidColor::new(albedo.r, albedo.g, albedo.b));
                    let center2 = center + vec3(0.0, rng.gen_range(0.0, 0.25), 0.0);
                    world.add(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(SolidColor::new(albedo.r, albedo.g, albedo.b), fuzz);
                    world.add(Sphere::new(center, 0.2, material));
                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1));
    world.add(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, mat2));
    world.add(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, mat3));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}

#[allow(dead_code)]
pub fn random_world_earth(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 0.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 12.0,
        time0: t0,
        time1: t1,
        background: color(0.7, 0.8, 1.0),
    });
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    let material_ground = Lambertian::new(SolidColor::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    ));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            let choose_mat: f64 = rng.gen::<f64>();
            let center = Vec3 {
                x: (a as f64) + 0.9 * rng.gen::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * rng.gen::<f64>(),
            };
            let some_point = vec3(4.0, 0.2, 0.0);

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.7 {
                    // diffuse
                    let albedo =
                        Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                    let material = Lambertian::new(SolidColor::new(albedo.r, albedo.g, albedo.b));
                    let center2 = center + vec3(0.0, rng.gen_range(0.0, 0.25), 0.0);
                    world.add(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        material.clone(),
                    ));
                } else if choose_mat < 0.90 {
                    // metal
                    let albedo = Color::random(0.5, 1.0, &mut rng);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let material = Metal::new(SolidColor::new(albedo.r, albedo.g, albedo.b), fuzz);
                    world.add(Sphere::new(center, 0.2, material.clone()));
                } else if choose_mat < 0.95 {
                    // earth
                    let material = Lambertian::new(ImageTexture::new("assets/earthmap.jpeg"));
                    world.add(Sphere::new(center, 0.2, material.clone()));
                } else {
                    // glass
                    let material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, material.clone()));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    world.add(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, mat2));
    world.add(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, mat3));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}
