use crate::camera::{Camera, CameraConfig};
use crate::color::color;
use crate::hittable::{bvh::BvhNode, hittable_list::HittableList, sphere::Sphere, Hittables};
use crate::material::lambertian::Lambertian;
use crate::scenes::Scene;
use crate::texture::{marble::MarbleTexture, noise::NoiseTexture, turbulence::TurbulenceTexture};
use crate::vec::vec3;

#[allow(dead_code)]
pub fn noise(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 2.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 10.0,
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });
    let texture = Lambertian::new(NoiseTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}

#[allow(dead_code)]
pub fn turbulence(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 2.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 10.0,
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });
    let texture = Lambertian::new(TurbulenceTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}

#[allow(dead_code)]
pub fn marble(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(13.0, 2.0, 3.0),
        lookat: vec3(0.0, 2.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 20.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: 10.0,
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });
    let texture = Lambertian::new(MarbleTexture::new(0, 4.0));
    let sphere1 = Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, texture.clone());
    let sphere2 = Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, texture.clone());
    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(sphere1);
    world.add(sphere2);
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1)),
        lights: Hittables::from(HittableList {
            hittables: Vec::new(),
        }),
    };
}
