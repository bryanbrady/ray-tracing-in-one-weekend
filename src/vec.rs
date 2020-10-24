use std::ops;
use rand::prelude::*;


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

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
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
        self.dot(*self)
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

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - 2.0*v.dot(n)*n;
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        Vec3{
            x: rand::thread_rng().gen_range(min,max),
            y: rand::thread_rng().gen_range(min,max),
            z: rand::thread_rng().gen_range(min,max)
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let u = Vec3::random(-1.0, 1.0);
            if u.length_squared() < 1.0 {
                return u;
            }

        }
    }

    // Lambertian distribution
    pub fn random_unit_vector() -> Vec3 {
        let a = rand::thread_rng().gen_range(0.0, 2.0*std::f64::consts::PI);
        let z = rand::thread_rng().gen_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z*z);
        return Vec3::new(r*f64::cos(a), r*f64::sin(a), z);
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let r = Vec3::random_in_unit_sphere();
        if r.dot(*normal) > 0.0 {
            return r;
        } else {
            return -r;
        }
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
