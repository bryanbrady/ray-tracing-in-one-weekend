use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::ray::{face_normal, Ray};
use crate::vec::vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct YzRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: Arc<MaterialType>,
}

#[allow(dead_code)]
impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<MaterialType>) -> Hittables {
        Hittables::from(YzRect {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
            mat: mat,
        })
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let normal = vec3(1.0, 0.0, 0.0);
        let (front_face, normal) = face_normal(ray, normal);
        Some(HitRecord {
            t: t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            point: ray.at(t),
            normal: normal,
            front_face: front_face,
            mat: self.mat.clone(),
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            vec3(self.k - 0.0001, self.y0, self.z0),
            vec3(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
