use crate::camera::Camera;
use crate::color::{color, Color};
use crate::hittable::{
    hittable_list::HittableList,
    sphere::{MovingSphere, Sphere},
    rect::XyRect,
    rect::XzRect,
    rect::YzRect,
    Hittables,
};
use crate::material::{
    dielectric::Dielectric, diffuse::Diffuse, lambertian::Lambertian, metal::Metal,
};
use crate::texture::{
    checker::CheckerTexture, image::ImageTexture, marble::MarbleTexture, noise::NoiseTexture,
    solidcolor::SolidColor, turbulence::TurbulenceTexture,
};
use crate::vec::{vec3, Vec3};
use crate::{ASPECT_RATIO, GRID_SIZE};
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub hittables: Hittables,
}

#[allow(dead_code)]
pub fn world1() -> HittableList {
    // World 1 - 5 spheres
    let material_ground = Lambertian::new(SolidColor::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(SolidColor::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(SolidColor::new(0.8, 0.6, 0.2), 0.0);
    let sphere1 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0, material_ground.clone());
    let sphere2 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5, material_center.clone());
    let sphere3 = Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere4 = Sphere::new(vec3(-1.0, 0.0, -1.0), 1.5, material_left.clone());
    let sphere5 = Sphere::new(vec3(1.0, 0.0, -1.0), 0.5, material_right.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);
    world.add(sphere5);
    return world;
}

#[allow(dead_code)]
pub fn world2() -> HittableList {
    // World 2 - 2 Spheres
    let r = f64::cos(std::f64::consts::PI / 4.0);
    let material_left = Lambertian::new(SolidColor::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(SolidColor::new(1.0, 0.0, 0.0));
    let sphere1 = Sphere::new(vec3(-r, 0.0, -1.0), r, material_left.clone());
    let sphere2 = Sphere::new(vec3(r, 0.0, -1.0), r, material_right.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return world;
}

#[allow(dead_code)]
pub fn noise1() -> HittableList {
    let texture = Lambertian::new(NoiseTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return world;
}

#[allow(dead_code)]
pub fn turbulence1() -> HittableList {
    let texture = Lambertian::new(TurbulenceTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return world;
}

#[allow(dead_code)]
pub fn marble1() -> HittableList {
    let texture = Lambertian::new(MarbleTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return world;
}

#[allow(dead_code)]
pub fn earth() -> HittableList {
    let earth_texture = Lambertian::new(ImageTexture::new("assets/earthmap.jpeg"));
    let texture = Lambertian::new(TurbulenceTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, earth_texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return world;
}

#[allow(dead_code)]
pub fn random_world_original() -> HittableList {
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
    return world;
}

#[allow(dead_code)]
pub fn random_checkered_world() -> HittableList {
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
    return world;
}

#[allow(dead_code)]
pub fn random_world() -> HittableList {
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
    return world;
}

#[allow(dead_code)]
pub fn random_world_earth() -> HittableList {
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

    // let difflight = Diffuse::new(SolidColor::new(4.0, 4.0, 4.0));
    // let rect = XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone());
    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(SolidColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    world.add(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, mat2));
    world.add(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, mat3));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    world.add(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    // world.add(rect);
    return world;
}

#[allow(dead_code)]
pub fn random_world2() -> HittableList {
    let mut rng = SmallRng::from_entropy();
    let mut world = HittableList {
        hittables: Vec::new(),
    };

    let material_ground = Lambertian::new(SolidColor::new(0.0, 0.0, 0.0));
    world.add(Sphere::new(
        vec3(0.0, -1100.0, 0.0),
        1000.0,
        material_ground,
    ));

    for a in -GRID_SIZE..GRID_SIZE {
        for b in -GRID_SIZE..GRID_SIZE {
            for c in -GRID_SIZE..GRID_SIZE {
                let choose_mat: f64 = rng.gen::<f64>();
                let center = Vec3 {
                    x: (a as f64) + 1.5 * rng.gen::<f64>(),
                    y: (b as f64) + 1.5 * rng.gen::<f64>(),
                    z: (c as f64) + 1.5 * rng.gen::<f64>(),
                };
                let some_point = vec3(4.0, 0.2, 0.0);

                if (center - some_point).length() > 0.9 {
                    if choose_mat < 0.2 {
                        // diffuse
                        let albedo =
                            Color::random(0.0, 1.0, &mut rng) * Color::random(0.0, 1.0, &mut rng);
                        let material =
                            Lambertian::new(SolidColor::new(albedo.r, albedo.g, albedo.b));
                        world.add(Sphere::new(center, 0.2, material));
                    } else if choose_mat < 0.75 {
                        // metal
                        let albedo = Color::random(0.5, 1.0, &mut rng);
                        let fuzz = rng.gen_range(0.0, 0.5);
                        let material =
                            Metal::new(SolidColor::new(albedo.r, albedo.g, albedo.b), fuzz);
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

    return world;
}

#[allow(dead_code)]
pub fn simple_light() -> HittableList {
    let texture = Lambertian::new(MarbleTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 2.0, 0.0), 2.0, texture.clone());

    let difflight = Diffuse::new(SolidColor::new(4.0, 4.0, 4.0));
    let rect = XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone());
    let sphere3 = Sphere::new(vec3(0.0, 7.0, 0.0), 2.0, difflight.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(rect);
    return world;
}

#[allow(dead_code)]
pub fn cornell_box() -> HittableList {
    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = Diffuse::new(SolidColor::new(15.0, 15.0, 15.0));

    let wall1 = YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone());
    let wall2 = YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone());
    let wall3 = XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let wall4 = XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let wall5 = XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());

    let light = XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone());

    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(wall1);
    world.add(wall2);
    world.add(wall3);
    world.add(wall4);
    world.add(wall5);
    world.add(light);
    return world;
}

#[allow(dead_code)]
pub fn camera2(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(3.0, 3.0, 2.0);
    let lookat = vec3(0.0, 0.0, -1.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 2.0;
    let dist_to_focus = (lookfrom - lookat).length();
    let background = color(0.7, 0.8, 1.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera3(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let background = color(0.7, 0.8, 1.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera_final(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 12.0;
    let background = color(0.7, 0.8, 1.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera_light(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 2.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let background = color(0.0, 0.0, 0.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera_cornell_box(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 40.0;
    let lookfrom = vec3(278.0, 278.0, -800.0);
    let lookat = vec3(278.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let background = color(0.0, 0.0, 0.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera_blur(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let background = color(0.7, 0.8, 1.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}

#[allow(dead_code)]
pub fn camera_other(t0: f64, t1: f64) -> Camera {
    let vfov: f64 = 20.0;
    let lookfrom = vec3(1.0, 20.0, 1.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 18.0;
    let background = color(0.7, 0.8, 1.0);
    return Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        t0,
        t1,
        background,
    );
}
