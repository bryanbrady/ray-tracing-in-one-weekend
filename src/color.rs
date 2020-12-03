use std::io::{self, Write};
use std::ops;
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::util::clamp;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
}

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color {
        r: x,
        g: y,
        b: z
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Self {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Self::Output {
        Color {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Self::Output {
        Color {
            r: self * _rhs.r,
            g: self * _rhs.g,
            b: self * _rhs.b
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Self::Output {
        Color {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b
        }
    }
}

impl Color {
    pub fn random(min: f64, max: f64, rng: &mut SmallRng) -> Color {
        Color {
            r: rng.gen_range(min,max),
            g: rng.gen_range(min,max),
            b: rng.gen_range(min,max)
        }
    }

}

pub fn write_color(_out: &mut impl Write, color: Color, samples_per_pixel: u64) -> io::Result<()> {
    let scale = 1.0 / (samples_per_pixel as f64);
    let c = color * scale;
    let ir = (255.999 * clamp(f64::sqrt(c.r), 0.0, 0.999)) as u64;
    let ig = (255.999 * clamp(f64::sqrt(c.g), 0.0, 0.999)) as u64;
    let ib = (255.999 * clamp(f64::sqrt(c.b), 0.0, 0.999)) as u64;
    let out = format!("{} {} {}\n", ir, ig, ib);
    _out.write_all(out.as_bytes())?;
    Ok(())
}
