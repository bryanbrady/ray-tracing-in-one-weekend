use crate::hittable::{aabb::Aabb, HitRecord, Hittable, Hittables};
use crate::ray::{face_normal, Ray};
use crate::util::degrees_to_radians;
use crate::vec::vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RotateX {
    pub object: Arc<Hittables>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

#[derive(Debug, Clone)]
pub struct RotateY {
    pub object: Arc<Hittables>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

#[derive(Debug, Clone)]
pub struct RotateZ {
    pub object: Arc<Hittables>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

impl RotateX {
    pub fn new(object: Arc<Hittables>, angle: f64) -> Hittables {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut min = vec3(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
        let mut max = vec3(
            -std::f64::INFINITY,
            -std::f64::INFINITY,
            -std::f64::INFINITY,
        );

        let bbox = match object.bounding_box(0.0, 1.0) {
            None => None,
            Some(bbox) => {
                for i in [0.0, 1.0].iter() {
                    for j in [0.0, 1.0].iter() {
                        for k in [0.0, 1.0].iter() {
                            let x = i * bbox.maximum.x + (1.0 - i) * bbox.minimum.x;
                            let y = j * bbox.maximum.y + (1.0 - j) * bbox.minimum.y;
                            let z = k * bbox.maximum.z + (1.0 - k) * bbox.minimum.z;
                            let newy = cos_theta * y - sin_theta * z;
                            let newz = sin_theta * y + cos_theta * z;
                            let tester = vec3(x, newy, newz);
                            min.x = min.x.min(tester.x);
                            min.y = min.y.min(tester.y);
                            min.z = min.z.min(tester.z);
                            max.x = max.x.max(tester.x);
                            max.y = max.y.max(tester.y);
                            max.z = max.z.max(tester.z);
                        }
                    }
                }
                Some(Aabb::new(min, max))
            }
        };

        Hittables::from(RotateX {
            object: object,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: bbox,
        })
    }
}

impl RotateY {
    pub fn new(object: Arc<Hittables>, angle: f64) -> Hittables {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut min = vec3(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
        let mut max = vec3(
            -std::f64::INFINITY,
            -std::f64::INFINITY,
            -std::f64::INFINITY,
        );

        let bbox = match object.bounding_box(0.0, 1.0) {
            None => None,
            Some(bbox) => {
                for i in [0.0, 1.0].iter() {
                    for j in [0.0, 1.0].iter() {
                        for k in [0.0, 1.0].iter() {
                            let x = i * bbox.maximum.x + (1.0 - i) * bbox.minimum.x;
                            let y = j * bbox.maximum.y + (1.0 - j) * bbox.minimum.y;
                            let z = k * bbox.maximum.z + (1.0 - k) * bbox.minimum.z;
                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;
                            let tester = vec3(newx, y, newz);
                            min.x = min.x.min(tester.x);
                            min.y = min.y.min(tester.y);
                            min.z = min.z.min(tester.z);
                            max.x = max.x.max(tester.x);
                            max.y = max.y.max(tester.y);
                            max.z = max.z.max(tester.z);
                        }
                    }
                }
                Some(Aabb::new(min, max))
            }
        };

        Hittables::from(RotateY {
            object: object,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: bbox,
        })
    }
}

impl RotateZ {
    pub fn new(object: Arc<Hittables>, angle: f64) -> Hittables {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut min = vec3(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY);
        let mut max = vec3(
            -std::f64::INFINITY,
            -std::f64::INFINITY,
            -std::f64::INFINITY,
        );

        let bbox = match object.bounding_box(0.0, 1.0) {
            None => None,
            Some(bbox) => {
                for i in [0.0, 1.0].iter() {
                    for j in [0.0, 1.0].iter() {
                        for k in [0.0, 1.0].iter() {
                            let x = i * bbox.maximum.x + (1.0 - i) * bbox.minimum.x;
                            let y = j * bbox.maximum.y + (1.0 - j) * bbox.minimum.y;
                            let z = k * bbox.maximum.z + (1.0 - k) * bbox.minimum.z;
                            let newx = cos_theta * x - sin_theta * y;
                            let newy = sin_theta * x + cos_theta * y;
                            let tester = vec3(newx, newy, z);
                            min.x = min.x.min(tester.x);
                            min.y = min.y.min(tester.y);
                            min.z = min.z.min(tester.z);
                            max.x = max.x.max(tester.x);
                            max.y = max.y.max(tester.y);
                            max.z = max.z.max(tester.z);
                        }
                    }
                }
                Some(Aabb::new(min, max))
            }
        };

        Hittables::from(RotateZ {
            object: object,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: bbox,
        })
    }
}

impl Hittable for RotateX {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.y = self.cos_theta * ray.origin.y + self.sin_theta * ray.origin.z;
        origin.z = -self.sin_theta * ray.origin.y + self.cos_theta * ray.origin.z;
        direction.y = self.cos_theta * ray.direction.y + self.sin_theta * ray.direction.z;
        direction.z = -self.sin_theta * ray.direction.y + self.cos_theta * ray.direction.z;

        let rotated = Ray {
            origin: origin,
            direction: direction,
            time: ray.time,
        };

        match self.object.hit(&rotated, t_min, t_max, rng) {
            None => None,
            Some(hit) => {
                let mut point = hit.point;
                let mut normal = hit.normal;
                point.y = self.cos_theta * hit.point.y - self.sin_theta * hit.point.z;
                point.z = self.sin_theta * hit.point.y + self.cos_theta * hit.point.z;
                normal.y = self.cos_theta * hit.normal.y - self.sin_theta * hit.normal.z;
                normal.z = self.sin_theta * hit.normal.y + self.cos_theta * hit.normal.z;

                let (front_face, normal) = face_normal(&rotated, normal);

                Some(HitRecord {
                    t: hit.t,
                    u: hit.u,
                    v: hit.v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: hit.mat.clone(),
                })
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;
        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated = Ray {
            origin: origin,
            direction: direction,
            time: ray.time,
        };

        match self.object.hit(&rotated, t_min, t_max, rng) {
            None => None,
            Some(hit) => {
                let mut point = hit.point;
                let mut normal = hit.normal;
                point.x = self.cos_theta * hit.point.x + self.sin_theta * hit.point.z;
                point.z = -self.sin_theta * hit.point.x + self.cos_theta * hit.point.z;
                normal.x = self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z;
                normal.z = -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z;

                let (front_face, normal) = face_normal(&rotated, normal);
                Some(HitRecord {
                    t: hit.t,
                    u: hit.u,
                    v: hit.v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: hit.mat.clone(),
                })
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}

impl Hittable for RotateZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x = self.cos_theta * ray.origin.x + self.sin_theta * ray.origin.y;
        origin.y = -self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.y;
        direction.x = self.cos_theta * ray.direction.x + self.sin_theta * ray.direction.y;
        direction.y = -self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.y;

        let rotated = Ray {
            origin: origin,
            direction: direction,
            time: ray.time,
        };

        match self.object.hit(&rotated, t_min, t_max, rng) {
            None => None,
            Some(hit) => {
                let mut point = hit.point;
                let mut normal = hit.normal;
                point.x = self.cos_theta * hit.point.x - self.sin_theta * hit.point.y;
                point.y = self.sin_theta * hit.point.x + self.cos_theta * hit.point.y;
                normal.x = self.cos_theta * hit.normal.x - self.sin_theta * hit.normal.y;
                normal.y = self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.y;

                let (front_face, normal) = face_normal(&rotated, normal);
                Some(HitRecord {
                    t: hit.t,
                    u: hit.u,
                    v: hit.v,
                    point: point,
                    normal: normal,
                    front_face: front_face,
                    mat: hit.mat.clone(),
                })
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
