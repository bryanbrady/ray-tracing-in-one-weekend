use crate::hittable::{
    aabb::Aabb,
    box3d::Box3D,
    bvh::BvhNode,
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
use std::sync::Arc;

pub mod aabb;
pub mod box3d;
pub mod bvh;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

#[enum_dispatch(Hittable)]
#[derive(Debug, Clone)]
pub enum Hittables {
    Box3D,
    BvhNode,
    MovingSphere,
    RotateX,
    RotateY,
    RotateZ,
    Sphere,
    Translate,
    HittableList,
    XyRect,
    XzRect,
    YzRect,
}
