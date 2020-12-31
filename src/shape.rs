use crate::hittable::{HitRecord, Hittable};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::sphere::{MovingSphere, Sphere};
use crate::vec::Vec3;

#[derive(Debug,Clone)]
pub enum Shape {
    Sphere(Sphere),
    MovingSphere(MovingSphere)
}

impl Hittable for Shape {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shape::Sphere(s)       => { s.hit(ray, t_min, t_max) },
            Shape::MovingSphere(s) => { s.hit(ray, t_min, t_max) }
        }
    }

}

#[allow(dead_code)]
pub fn sphere(center: Vec3, radius: f64, mat: MaterialType) -> Shape {
    Shape::Sphere(Sphere{
        center: center,
        radius: radius,
        mat: mat
    })
}

#[allow(dead_code)]
pub fn moving_sphere(center0: Vec3, center1: Vec3, t0: f64, t1: f64, radius: f64, mat: MaterialType) -> Shape {
    Shape::MovingSphere(MovingSphere{
        center0: center0,
        center1: center1,
        radius: radius,
        time0: t0,
        time1: t1,
        mat: mat
    })
}
