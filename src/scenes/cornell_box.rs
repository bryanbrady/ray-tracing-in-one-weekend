use crate::camera::{Camera, CameraConfig};
use crate::color::color;
use crate::hittable::{
    box3d::Box3D,
    bvh::BvhNode,
    flip_face::FlipFace,
    hittable_list::HittableList,
    rect::{XyRect, XzRect, YzRect},
    rotate::RotateY,
    translate::Translate,
    Hittables,
};
use crate::material::{
    diffuse::Diffuse,
    lambertian::Lambertian,
};
use crate::scenes::Scene;
use crate::texture::solidcolor::SolidColor;
use crate::vec::vec3;

use std::sync::Arc;

#[allow(dead_code)]
pub fn cornell_box(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(278.0, 278.0, -800.0),
        lookat: vec3(278.0, 278.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 40.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.0,
        focus_dist: 10.0,
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = Diffuse::new(SolidColor::new(15.0, 15.0, 15.0));

    let wall1 = YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone());
    let wall2 = YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone());
    let wall3 = XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let wall4 = XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let wall5 = XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());

    let box1 = Box3D::new(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), vec3(265.0, 0.0, 295.0));

    let box2 = Box3D::new(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), vec3(130.0, 0.0, 65.0));

    let light = XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone());

    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(wall1);
    world.add(wall2);
    world.add(wall3);
    world.add(wall4);
    world.add(wall5);
    world.add(box1);
    world.add(box2);
    world.add(light);
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1))
    };
}

#[allow(dead_code)]
pub fn cornell_box_test(t0: f64, t1: f64, aspect_ratio: f64) -> Scene {
    let camera = Camera::new(CameraConfig {
        lookfrom: vec3(278.0, 278.0, -800.0),
        lookat: vec3(278.0, 278.0, 0.0),
        vup: vec3(0.0, 1.0, 0.0),
        vfov: 40.0,
        aspect_ratio: aspect_ratio,
        aperture: 0.0,
        focus_dist: 10.0,
        time0: t0,
        time1: t1,
        background: color(0.0, 0.0, 0.0),
    });

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = Diffuse::new(SolidColor::new(15.0, 15.0, 15.0));

    let wall1 = YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone());
    let wall2 = YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red.clone());
    let wall3 = XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone());
    let wall4 = XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());
    let wall5 = XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone());

    let box1 = Box3D::new(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Arc::new(box1), 15.0);
    let box1 = Translate::new(Arc::new(box1), vec3(265.0, 0.0, 295.0));

    let box2 = Box3D::new(
        vec3(0.0, 0.0, 0.0),
        vec3(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = RotateY::new(Arc::new(box2), -18.0);
    let box2 = Translate::new(Arc::new(box2), vec3(130.0, 0.0, 65.0));

    let light = FlipFace::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone()));

    let mut world = HittableList {
        hittables: Vec::new(),
    };
    world.add(wall1);
    world.add(wall2);
    world.add(wall3);
    world.add(wall4);
    world.add(wall5);
    world.add(box1);
    world.add(box2);
    world.add(light);
    return Scene {
        camera: camera,
        hittables: Hittables::from(BvhNode::new(world, t0, t1))
    };
}
