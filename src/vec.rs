use std::ops;

#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// TODO: Learn how to use macros
// TODO: and if they're needed when Debug,Clone,Copy is used

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs
        }
    }

}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self + _rhs.x,
            y: self + _rhs.y,
            z: self + _rhs.z
        }
    }

}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z
        }
    }

}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Self::Output {
        self * (1.0 / _rhs)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    #[allow(dead_code)]
    pub fn dot(self, _rhs: Vec3) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

    #[allow(dead_code)]
    pub fn cross(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x
        }
    }

    #[allow(dead_code)]
    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();
        self / len
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_add() {
        let a = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let c = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        let ab = a + b;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn vec_sub() {
        let a = Vec3 { x: 10.0, y: 10.0, z: 10.0 };
        let b = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
        let c = Vec3 { x: 9.0, y: 8.0, z: 7.0 };
        let ab = a - b;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_add() {
        let a = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        let c = Vec3 { x: 3.0, y: 3.0, z: 3.0 };
        let ab = a + 2.0;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_sub() {
        let a = Vec3 { x: 10.0, y: 10.0, z: 10.0 };
        let c = Vec3 { x: 9.0, y: 9.0, z: 9.0 };
        let ab = a - 1.0;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_mul() {
        let a = Vec3 { x: 10.0, y: 10.0, z: 10.0 };
        let c = Vec3 { x: 9.0, y: 9.0, z: 9.0 };
        let ab = a * 0.9;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn dot_unit() {
        let a = Vec3 { x: 3.0, y: 4.0, z: 5.0 };
        let c = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!(12.0, a.dot(c))
    }
}
