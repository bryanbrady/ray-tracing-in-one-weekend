use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::ray::{face_normal, Ray};
use crate::vec::{vec3, Vec3};
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct XyRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat: Arc<MaterialType>,
}

#[allow(dead_code)]
impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<MaterialType>) -> Hittables {
        Hittables::from(XyRect {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            mat: mat,
        })
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut SmallRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let normal = vec3(0.0, 0.0, 1.0);
        let (front_face, normal) = face_normal(ray, normal);
        Some(HitRecord {
            t: t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            point: ray.at(t),
            normal: normal,
            front_face: front_face,
            mat: self.mat.clone(),
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            vec3(self.x0, self.y0, self.k - 0.0001),
            vec3(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut SmallRng) -> Option<HitRecord> {
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

    fn pdf_value(&self, origin: Vec3, v: Vec3, rng: &mut SmallRng) -> f64 {
        let ray = Ray {
            origin: origin,
            direction: v,
            time: 0.0, // arbitrary
        };
        match self.hit(&ray, 0.001, std::f64::INFINITY, rng) {
            None => { return 0.0; },
            Some(hit) => {
                let area = (self.x1-self.x0) * (self.z1-self.z0);
                let distance_squared = hit.t * hit.t * v.length_squared();
                let cosine = f64::abs(v.dot(hit.normal) / v.length());
                return distance_squared / (cosine * area);
            }
        }
    }

    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let random_point = Vec3 {
            x: rng.gen_range(self.x0, self.x1),
            y: self.k,
            z: rng.gen_range(self.z0, self.z1),
        };
        return random_point - origin;
    }
}

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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut SmallRng) -> Option<HitRecord> {
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
