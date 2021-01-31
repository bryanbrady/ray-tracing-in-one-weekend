use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::{isotropic::Isotropic, MaterialType};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::util::random_double;
use crate::vec::vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    pub boundary: Arc<Hittables>,
    pub phase_fn: Arc<MaterialType>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<Hittables>, density: f64, texture: Texture) -> Hittables {
        Hittables::from(ConstantMedium {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_fn: Isotropic::new(texture),
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut rec1 = match self
            .boundary
            .hit(ray, -std::f64::INFINITY, std::f64::INFINITY, rng)
        {
            Some(hit) => hit,
            None => {
                return None;
            }
        };
        let mut rec2 = match self
            .boundary
            .hit(ray, rec1.t + 0.0001, std::f64::INFINITY, rng)
        {
            Some(hit) => hit,
            None => {
                return None;
            }
        };
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(rng).ln();
        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        Some(HitRecord {
            t: t,
            u: 0.0, // arbitrary
            v: 0.0, // arbitrary
            point: ray.at(t),
            normal: vec3(1.0, 0.0, 0.0), // arbitrary
            front_face: true,            // arbitrary
            mat: self.phase_fn.clone(),
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
