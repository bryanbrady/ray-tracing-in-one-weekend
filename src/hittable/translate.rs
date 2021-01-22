use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::ray::{face_normal, Ray};
use crate::vec::Vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Translate {
    pub object: Arc<Hittables>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<Hittables>, offset: Vec3) -> Hittables {
        Hittables::from(Translate {
            object: object,
            offset: offset,
        })
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray {
            origin: ray.origin - self.offset,
            direction: ray.direction,
            time: ray.time,
        };
        match self.object.hit(&moved, t_min, t_max) {
            Some(hit) => {
                let (front_face, normal) = face_normal(&moved, hit.normal);
                Some(HitRecord {
                    t: hit.t,
                    u: hit.u,
                    v: hit.v,
                    point: hit.point + self.offset,
                    normal: normal,
                    front_face: front_face,
                    mat: hit.mat.clone(),
                })
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self.object.bounding_box(time0, time1) {
            Some(bbox) => Some(Aabb::new(
                bbox.minimum + self.offset,
                bbox.maximum + self.offset,
            )),
            None => None,
        }
    }
}
