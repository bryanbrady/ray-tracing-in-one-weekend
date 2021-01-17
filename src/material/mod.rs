use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter>;
}

#[enum_dispatch(Material)]
#[derive(Debug, Clone)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

impl Default for MaterialType {
    fn default() -> MaterialType {
        Lambertian::new(Texture::default())
    }
}
