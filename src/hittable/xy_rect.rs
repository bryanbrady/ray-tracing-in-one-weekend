use std::sync::Arc;
use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::ray::{face_normal, Ray};
use crate::vec::vec3;

#[derive(Debug, Clone)]
pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat: Arc<MaterialType>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<MaterialType>) -> Hittables {
        Hittables::from(XyRect { x0: x0, x1: x1, y0: y0, y1: y1, k: k, mat: mat })
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k-ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max { return None; }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 { return None; }
        let normal = vec3(0.0, 0.0, 1.0);
        let (front_face, normal) = face_normal(ray, normal);
        Some(HitRecord{
            t: t,
            u: (x-self.x0)/(self.x1-self.x0),
            v: (y-self.y0)/(self.y1-self.y0),
            point: ray.at(t),
            normal: normal,
            front_face: front_face,
            mat: self.mat.clone(),
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(vec3(self.x0, self.y0, self.k-0.0001), vec3(self.x1, self.y1, self.k+0.0001)))
    }
}

