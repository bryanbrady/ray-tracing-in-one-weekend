use crate::vec::Vec3;

#[derive(Debug,Clone,Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    #[allow(dead_code)]
    pub fn at(self, _t: f64) -> Vec3 {
        self.origin + self.direction * _t
    }
}

