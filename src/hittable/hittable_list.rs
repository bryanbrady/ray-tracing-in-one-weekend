use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::ray::Ray;
use crate::vec::Vec3;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug, Clone)]
pub struct HittableList {
    pub hittables: Vec<Hittables>,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new() -> Hittables {
        Hittables::from(HittableList {
            hittables: Vec::new(),
        })
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.hittables.clear()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, s: Hittables) {
        self.hittables.push(s)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;
        let mut closest = t_max;
        for hittable in self.hittables.iter() {
            match hittable.hit(ray, t_min, closest, rng) {
                Some(h) => {
                    closest = h.t;
                    ret = Some(h);
                }
                None => (),
            }
        }
        return ret;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.hittables.is_empty() {
            return None;
        }
        let mut temp_box = Aabb::default();
        for hittable in self.hittables.iter() {
            match hittable.bounding_box(time0, time1) {
                Some(b) => {
                    temp_box = Aabb::surrounding_box(temp_box, b);
                }
                None => {
                    return None;
                }
            }
        }
        Some(temp_box)
    }

    fn pdf_value(&self, origin: Vec3, v: Vec3, rng: &mut SmallRng) -> f64 {
        let weight = 1.0/(self.hittables.len() as f64);
        let mut sum = 0.0;

        for h in self.hittables.iter() {
            sum += weight * h.pdf_value(origin, v, rng);
        }
         return sum;
    }

    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let sz = self.hittables.len();
        return self.hittables[rng.gen_range(0, sz)].random(origin, rng);
    }
}
