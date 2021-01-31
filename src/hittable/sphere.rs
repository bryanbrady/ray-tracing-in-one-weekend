use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::onb::Onb;
use crate::ray::{face_normal, Ray};
use crate::util::random_to_sphere;
use crate::vec::{vec3, Vec3};
use rand::rngs::SmallRng;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Arc<MaterialType>,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f64, mat: Arc<MaterialType>) -> Hittables {
        Hittables::from(Sphere {
            center: center,
            radius: radius,
            mat: mat,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut SmallRng) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp1 = (-half_b - root) / a;
            if temp1 < t_max && temp1 > t_min {
                let t = temp1;
                let point = ray.at(temp1);
                let normal = (point - self.center) / self.radius;
                let (front_face, normal) = face_normal(ray, normal);
                let (u, v) = get_sphere_uv(&normal);

                return Some(HitRecord {
                    t: t,
                    u: u,
                    v: v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: self.mat.clone(),
                });
            }

            let temp2 = (-half_b + root) / a;
            if temp2 < t_max && temp2 > t_min {
                let t = temp2;
                let point = ray.at(temp2);
                let normal = (point - self.center) / self.radius;
                let (front_face, normal) = face_normal(ray, normal);
                let (u, v) = get_sphere_uv(&normal);

                return Some(HitRecord {
                    t: t,
                    u: u,
                    v: v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: self.mat.clone(),
                });
            }
        }
        return None;
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb {
            minimum: self.center - vec3(self.radius, self.radius, self.radius),
            maximum: self.center + vec3(self.radius, self.radius, self.radius),
        })
    }

    fn pdf_value(&self, origin: Vec3, v: Vec3, rng: &mut SmallRng) -> f64 {
        let ray = Ray {
            origin: origin,
            direction: v,
            time: 0.0,
        };
        let hit = self.hit(&ray, 0.001, std::f64::INFINITY, rng);
        match hit {
            None => {
                return 0.0;
            }
            Some(_) => {
                let cos_theta_max = f64::sqrt(
                    1.0 * self.radius * self.radius / (self.center - origin).length_squared(),
                );
                let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
                return 1.0 / solid_angle;
            }
        }
    }

    fn random(&self, origin: Vec3, rng: &mut SmallRng) -> Vec3 {
        let direction = self.center - origin;
        let dist_squared = direction.length_squared();
        let uvw = Onb::new(&direction);
        return uvw.local(&random_to_sphere(self.radius, dist_squared, rng));
    }
}

#[derive(Debug, Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat: Arc<MaterialType>,
}

impl MovingSphere {
    #[allow(dead_code)]
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f64,
        t1: f64,
        radius: f64,
        mat: Arc<MaterialType>,
    ) -> Hittables {
        Hittables::from(MovingSphere {
            center0: center0,
            center1: center1,
            time0: t0,
            time1: t1,
            radius: radius,
            mat: mat,
        })
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _rng: &mut SmallRng) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let temp1 = (-half_b - root) / a;
            if temp1 < t_max && temp1 > t_min {
                let t = temp1;
                let point = ray.at(temp1);
                let normal = (point - self.center(ray.time)) / self.radius;
                let (front_face, normal) = face_normal(ray, normal);
                let (u, v) = get_sphere_uv(&normal);

                return Some(HitRecord {
                    t: t,
                    u: u,
                    v: v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: self.mat.clone(),
                });
            }

            let temp2 = (-half_b + root) / a;
            if temp2 < t_max && temp2 > t_min {
                let t = temp2;
                let point = ray.at(temp2);
                let normal = (point - self.center(ray.time)) / self.radius;
                let (front_face, normal) = face_normal(ray, normal);
                let (u, v) = get_sphere_uv(&normal);

                return Some(HitRecord {
                    t: t,
                    u: u,
                    v: v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: self.mat.clone(),
                });
            }
        }
        return None;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            minimum: self.center(time0) - vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time0) + vec3(self.radius, self.radius, self.radius),
        };
        let box1 = Aabb {
            minimum: self.center(time1) - vec3(self.radius, self.radius, self.radius),
            maximum: self.center(time1) + vec3(self.radius, self.radius, self.radius),
        };
        Some(Aabb::surrounding_box(box0, box1))
    }
}

pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
    let theta = f64::acos(-p.y);
    let phi = f64::atan2(-p.z, p.x) + std::f64::consts::PI;
    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    (u, v)
}
