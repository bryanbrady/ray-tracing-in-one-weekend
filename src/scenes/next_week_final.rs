use crate::camera::{Camera, CameraConfig};
use crate::color::color;
use crate::hittable::{
    box3d::Box3D,
    bvh::BvhNode,
    constant_medium::ConstantMedium,
    hittable_list::HittableList,
    rect::XzRect,
    rotate::RotateY,
    sphere::{MovingSphere, Sphere},
    translate::Translate,
    Hittables,
};
use crate::material::{
    dielectric::Dielectric,
    diffuse::Diffuse,
    lambertian::Lambertian,
    metal::Metal,
};
use crate::scenes::Scene;
use crate::texture::{
    image::ImageTexture,
    noise::NoiseTexture,
    solidcolor::SolidColor,
};
use crate::vec::{vec3, Vec3};
use crate::util::random_double;

use rand::prelude::*;
use rand::rngs::SmallRng;
use std::sync::Arc;

#[allow(dead_code)]
pub fn next_week_final(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let lookfrom =  vec3(478.0, 278.0, -600.0);
    let lookat = vec3(278.0, 278.0, 0.0);
    let camera = Camera::new(CameraConfig {
        lookfrom: lookfrom,
        lookat: lookat,
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 40.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.1,
        focus_dist: (lookfrom - lookat).length(),
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });
    let mut rng = SmallRng::seed_from_u64(0);
    let mut boxes1 = HittableList {
        hittables: Vec::new(),
    };
    let ground = Lambertian::new(SolidColor::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * random_double(&mut rng);
            let z1 = z0 + w;
            boxes1.add(Box3D::new(vec3(x0, y0, z0), vec3(x1, y1, z1), ground.clone()));
        }
    }

    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(Hittables::from(BvhNode::new(boxes1, 0.0, 1.0)));

    let light = Diffuse::new(SolidColor::new(7.0, 7.0, 7.0));
    world.add(XzRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone()));

    let center1 = vec3(400.0, 400.0, 200.0);
    let center2 = center1 + vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColor::new(0.7, 0.3, 0.1));
    let moving_sphere = MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone());
    world.add(moving_sphere);

    let dielectric1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(vec3(260.0, 150.0, 45.0), 50.0, dielectric1.clone());

    let metal1 = Metal::new(SolidColor::new(0.8, 0.8, 0.9), 1.0);
    let sphere2 = Sphere::new(vec3(0.0, 150.0, 145.0), 50.0, metal1.clone());

    let earth_texture = Lambertian::new(ImageTexture::new("assets/earthmap.jpeg"));
    let sphere3 = Sphere::new(vec3(400.0, 200.0, 400.0), 100.0, earth_texture.clone());

    let noise1 = Lambertian::new(NoiseTexture::new(0, 0.1));
    let sphere4 = Sphere::new(vec3(220.0, 280.0, 300.0), 80.0, noise1.clone());

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);

    let boundary1 = Sphere::new(vec3(360.0, 150.0, 145.0), 70.0, dielectric1.clone());
    world.add(boundary1.clone());
    world.add(ConstantMedium::new(Arc::new(boundary1), 0.2, SolidColor::new(0.2, 0.4, 0.9)));

    let boundary2 = Sphere::new(Vec3::zero(), 5000.0, dielectric1.clone());
    world.add(ConstantMedium::new(Arc::new(boundary2), 0.0001, SolidColor::new(1.0, 1.0, 1.0)));

    let mut boxes2 = HittableList {
        hittables: Vec::new(),
    };
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _ in 0..ns{
        let sphere = Sphere::new(Vec3::random(0.0, 165.0, &mut rng), 10.0, white.clone());
        boxes2.add(sphere);
    }

    let bvh = Hittables::from(BvhNode::new(boxes2, 0.0, 1.0));
    let bvh = RotateY::new(Arc::new(bvh), 15.0);
    let bvh = Translate::new(Arc::new(bvh), vec3(-100.0, 270.0, 395.0));
    world.add(bvh);

    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1))
    };
}
