
use crate::hittable::{HitRecord, Hittable};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug,Clone,Copy)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3
}

impl Aabb {
    pub fn new(_min: Vec3, _max: Vec3) -> Aabb {
        Aabb { minimum: _min, maximum: _max }
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let small = Vec3 {
            x: box0.minimum.x.min(box1.minimum.x),
            y: box0.minimum.y.min(box1.minimum.y),
            z: box0.minimum.z.min(box1.minimum.z)
        };
        let large = Vec3 {
            x: box0.maximum.x.max(box1.maximum.x),
            y: box0.maximum.y.max(box1.maximum.y),
            z: box0.maximum.z.max(box1.maximum.z)
        };
        Aabb { minimum: small, maximum: large }
    }
}

impl Default for Aabb {
    fn default() -> Aabb {
        Aabb { minimum: Vec3::zero(), maximum: Vec3::zero() }
    }
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // x
        let inv_d = 1.0 / ray.direction.x;
        let t0 = (self.minimum.x - ray.origin.x) * inv_d;
        let t1 = (self.maximum.x - ray.origin.x) * inv_d;
        let (t0, t1) = if inv_d >= 0.0 { (t0, t1) } else { (t1, t0) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return None }

        // y
        let inv_d = 1.0 / ray.direction.y;
        let t0 = (self.minimum.y - ray.origin.y) * inv_d;
        let t1 = (self.maximum.y - ray.origin.y) * inv_d;
        let (t0, t1) = if inv_d >= 0.0 { (t0, t1) } else { (t1, t0) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return None }

        // z
        let inv_d = 1.0 / ray.direction.z;
        let t0 = (self.minimum.z - ray.origin.z) * inv_d;
        let t1 = (self.maximum.z - ray.origin.z) * inv_d;
        let (t0, t1) = if inv_d >= 0.0 { (t0, t1) } else { (t1, t0) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min { return None }

        // Return dummy HitRecord
        return Some(HitRecord {
            t: 0.0,
            point: Vec3::default(),
            normal: Vec3::default(),
            front_face: false,
            mat: MaterialType::default()
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(*self)
    }
}