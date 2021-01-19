use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::ray::{face_normal, Ray};
use crate::vec::vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct XzRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: Arc<MaterialType>,
}

#[allow(dead_code)]
impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<MaterialType>) -> Hittables {
        Hittables::from(XzRect {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
            mat: mat,
        })
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let normal = vec3(0.0, 1.0, 0.0);
        let (front_face, normal) = face_normal(ray, normal);
        Some(HitRecord {
            t: t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            point: ray.at(t),
            normal: normal,
            front_face: front_face,
            mat: self.mat.clone(),
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            vec3(self.x0, self.k - 0.0001, self.z0),
            vec3(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
