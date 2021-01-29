use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::ray::Ray;
use rand::rngs::SmallRng;

#[derive(Debug, Clone)]
pub struct FlipFace{
    pub object: Box<Hittables>,
}

impl FlipFace {
    pub fn new(object: Hittables) -> Hittables {
        Hittables::from(FlipFace {
            object: Box::new(object)
        })
    }

}

impl Hittable for FlipFace{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        match  self.object.hit(&ray, t_min, t_max, rng) {
            None => { return None; }
            Some(hit) => {
                return Some(HitRecord{
                    point: hit.point,
                    normal: hit.normal,
                    t: hit.t,
                    u: hit.u,
                    v: hit.v,
                    front_face: !hit.front_face,
                    mat: hit.mat.clone(),
                });
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.object.bounding_box(time0, time1)
    }
}
