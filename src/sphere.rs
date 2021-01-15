use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable, Hittables};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec::{vec3, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: MaterialType,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f64, mat: MaterialType) -> Hittables {
        Hittables::from(Sphere {
            center: center,
            radius: radius,
            mat: mat,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                let outward_normal = (point - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                let (u, v) = get_sphere_uv(&point);

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
                let outward_normal = (point - self.center) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                let (u, v) = get_sphere_uv(&point);

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
}

#[derive(Debug, Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
    pub mat: MaterialType,
}

impl MovingSphere {
    #[allow(dead_code)]
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f64,
        t1: f64,
        radius: f64,
        mat: MaterialType,
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                let outward_normal = (point - self.center(ray.time)) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                let (u, v) = get_sphere_uv(&point);

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
                let outward_normal = (point - self.center(ray.time)) / self.radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                let (u, v) = get_sphere_uv(&point);

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
    let phi = f64::atan2(-p.z, p.x + std::f64::consts::PI);
    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    (u, v)
}
