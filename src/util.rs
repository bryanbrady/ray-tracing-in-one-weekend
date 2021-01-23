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
