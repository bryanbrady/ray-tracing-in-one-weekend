use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::{Vec3};

#[derive(Debug,Clone,Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere{
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp1 = (-half_b - root) / a;
            if temp1 < t_max && temp1 > t_min {
                let t = temp1;
                let point = ray.at(temp1);
                let outward_normal = (point - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { -outward_normal };

                return Some(HitRecord {
                    t: t,
                    point: point,
                    normal: normal,
                    front_face: front_face
                })
            }

            let temp2 = (-half_b + root) / a;
            if temp2 < t_max && temp2 > t_min {
                let t = temp2;
                let point = ray.at(temp2);
                let outward_normal = (point - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { -outward_normal };

                return Some(HitRecord {
                    t: t,
                    point: point,
                    normal: normal,
                    front_face: front_face
                })
            }
        }
        return None
    }

}
