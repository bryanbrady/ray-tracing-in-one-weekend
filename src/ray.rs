use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    #[allow(dead_code)]
    pub fn at(self, _t: f64) -> Vec3 {
        self.origin + self.direction * _t
    }
}

pub fn face_normal(ray: &Ray, normal: Vec3) -> (bool, Vec3) {
    let front_face = ray.direction.dot(normal) < 0.0;
    if front_face {
        (true, normal)
    } else {
        (false, -normal)
    }
}
