use enum_dispatch::enum_dispatch;
use crate::aabb::Aabb;
use crate::hittable_list::HittableList;
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::sphere::{Sphere, MovingSphere};
use crate::vec::{Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: MaterialType
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

#[enum_dispatch(Hittable)]
#[derive(Debug, Clone)]
pub enum Hittables {
    MovingSphere,
    Sphere,
    HittableList
}
