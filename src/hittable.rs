use crate::material::MaterialType;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec::{Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: MaterialType
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug,Clone)]
pub struct HittableList {
    pub hittables : Vec<Shape>
}

impl HittableList {

    #[allow(dead_code)]
    pub fn new() -> HittableList{
        HittableList {
            hittables: Vec::new()
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.hittables.clear()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, s: Shape) {
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
}
