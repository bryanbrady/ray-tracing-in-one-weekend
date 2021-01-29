use crate::vec::{vec3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: &Vec3) -> Onb {
        let axis2 = n.unit_vector();
        let a = if axis2.x.abs() > 0.9 { vec3(0.0, 1.0, 0.0) } else { vec3(1.0, 0.0, 0.0) };
        let axis1 = axis2.cross(a).unit_vector();
        let axis0 = axis2.cross(axis1);
        Onb { axis: [axis0, axis1, axis2] }
    }

    pub fn u(&self) -> Vec3 { return self.axis[0]; }
    pub fn v(&self) -> Vec3 { return self.axis[1]; }
    pub fn w(&self) -> Vec3 { return self.axis[2]; }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        return a.x * self.u() + a.y * self.v() + a.z * self.w();
    }
}
