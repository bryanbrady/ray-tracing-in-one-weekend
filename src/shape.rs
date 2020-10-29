use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::Vec3;

#[derive(Debug,Clone)]
pub enum Shape {
    Sphere(Sphere)
}

impl Hittable for Shape {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shape::Sphere(s) => { s.hit(ray, t_min, t_max) }
        }
    }

}

#[allow(dead_code)]
pub fn sphere(center: Vec3, radius: f64, mat: Material) -> Shape {
    Shape::Sphere(Sphere{
        center: center,
        radius: radius,
        mat: mat
    })
}
