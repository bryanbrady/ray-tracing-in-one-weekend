use crate::aabb::Aabb;
use crate::hittable::{HitRecord,Hittable,Hittables};
use crate::ray::Ray;

#[derive(Debug,Clone)]
pub struct HittableList {
    pub hittables : Vec<Hittables>
}

impl HittableList {

    #[allow(dead_code)]
    pub fn new() -> Hittables{
        Hittables::from(HittableList {
            hittables: Vec::new()
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
         let mut ret : Option<HitRecord> = None;
         let mut closest = t_max;
         for hittable in self.hittables.iter() {
             match hittable.hit(ray, t_min, closest) {
                 Some(h) => {
                     closest = h.t;
                     ret = Some(h);
                 },
                 None => ()
             }
         }
         return ret
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.hittables.is_empty() { return None; }
        let mut temp_box = Aabb::default();
        for hittable in self.hittables.iter() {
            match hittable.bounding_box(time0, time1) {
                Some(b) => {
                    temp_box = Aabb::surrounding_box(temp_box, b);
                },
                None => {
                    return None;
                }
            }
        }
        Some(temp_box)
    }
}
