use crate::hittable::{
    aabb::Aabb,
    box3d::Box3D,
    bvh::BvhNode,
    constant_medium::ConstantMedium,
    flip_face::FlipFace,
    hittable_list::HittableList,
    rect::{XyRect, XzRect, YzRect},
    rotate::{RotateX, RotateY, RotateZ},
    sphere::{MovingSphere, Sphere},
    translate::Translate,
};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec::Vec3;
use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;
use std::sync::Arc;

pub mod aabb;
pub mod box3d;
pub mod bvh;
pub mod constant_medium;
pub mod flip_face;
pub mod hittable_list;
pub mod rect;
pub mod rotate;
pub mod sphere;
pub mod translate;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<MaterialType>,
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _origin: Vec3, _v: Vec3, _rng: &mut SmallRng) -> f64 { 0.0 }
    fn random(&self, _origin: Vec3, _rng: &mut SmallRng) -> Vec3 { Vec3::new(1.0, 0.0, 0.0) }
}

#[enum_dispatch(Hittable)]
#[derive(Debug, Clone)]
pub enum Hittables {
    Box3D,
    BvhNode,
    ConstantMedium,
    FlipFace,
    HittableList,
    MovingSphere,
    RotateX,
    RotateY,
    RotateZ,
    Sphere,
    Translate,
    XyRect,
    XzRect,
    YzRect,
}
