use crate::vec::{vec3, Vec3};
use rand::prelude::*;
use rand::rngs::SmallRng;

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn random_double(rng: &mut SmallRng) -> f64 {
    rng.gen()
}

pub fn random_to_sphere(radius: f64, dist_squared: f64, rng: &mut SmallRng) -> Vec3 {
    let r1 = random_double(rng);
    let r2 = random_double(rng);
    let phi = 2.0 * std::f64::consts::PI * r1;
    let z = 1.0 + r2 * (f64::sqrt(1.0 - radius * radius / dist_squared) - 1.0);
    let x = f64::cos(phi) * f64::sqrt(1.0 - z * z);
    let y = f64::sin(phi) * f64::sqrt(1.0 - z * z);
    vec3(x, y, z)
}
