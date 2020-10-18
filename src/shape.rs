use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;

#[derive(Debug,Clone,Copy)]
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

