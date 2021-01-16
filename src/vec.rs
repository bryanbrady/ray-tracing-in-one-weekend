use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::zero()
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn dot(self, _rhs: Vec3) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }

    pub fn cross(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x,
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();
        self / len
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        f64::abs(self.x) < s && f64::abs(self.y) < s && f64::abs(self.z) < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v - 2.0 * v.dot(n) * n;
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -uv.dot(n);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
        return r_out_perp + r_out_parallel;
    }

    pub fn random(min: f64, max: f64, rng: &mut SmallRng) -> Vec3 {
        Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Vec3 {
        loop {
            let u = Vec3::random(-1.0, 1.0, rng);
            if u.length_squared() < 1.0 {
                return u;
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut SmallRng) -> Vec3 {
        loop {
            let p = Vec3 {
                x: rng.gen_range(-1.0, 1.0),
                y: rng.gen_range(-1.0, 1.0),
                z: 0.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    // Lambertian distribution
    pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
        let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);
        return Vec3::new(r * f64::cos(a), r * f64::sin(a), z);
    }

    pub fn random_in_hemisphere(normal: &Vec3, rng: &mut SmallRng) -> Vec3 {
        let r = Vec3::random_in_unit_sphere(rng);
        if r.dot(*normal) > 0.0 {
            return r;
        } else {
            return -r;
        }
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x + _rhs,
            y: self.y + _rhs,
            z: self.z + _rhs,
        }
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self + _rhs.x,
            y: self + _rhs.y,
            z: self + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
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
            z: -self.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        *self = Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_add() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let c = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let ab = a + b;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn vec_sub() {
        let a = Vec3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let c = Vec3 {
            x: 9.0,
            y: 8.0,
            z: 7.0,
        };
        let ab = a - b;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_add() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let c = Vec3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        };
        let ab = a + 2.0;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_sub() {
        let a = Vec3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        };
        let c = Vec3 {
            x: 9.0,
            y: 9.0,
            z: 9.0,
        };
        let ab = a - 1.0;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn const_mul() {
        let a = Vec3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        };
        let c = Vec3 {
            x: 9.0,
            y: 9.0,
            z: 9.0,
        };
        let ab = a * 0.9;
        assert_eq!(c.x, ab.x);
        assert_eq!(c.y, ab.y);
        assert_eq!(c.z, ab.z);
    }

    #[test]
    fn dot_unit() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let c = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(12.0, a.dot(c))
    }
}
