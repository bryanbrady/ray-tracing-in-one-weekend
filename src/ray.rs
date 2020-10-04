use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3
}

impl Ray {
    #[allow(dead_code)]
    pub fn at(self, _t: f64) -> vec::Vec3 {
        self.origin + self.direction * _t
    }
}

