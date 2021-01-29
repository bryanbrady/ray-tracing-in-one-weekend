use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{solidcolor::SolidColor, Texture};
use crate::vec::Vec3;
use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;

use crate::material::dielectric::Dielectric;
use crate::material::diffuse::Diffuse;
use crate::material::isotropic::Isotropic;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

pub mod dielectric;
pub mod diffuse;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

#[derive(Debug)]
pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
    pub pdf: f64,
}

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter>;
    fn scattering_pdf(&self, ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f64;
    fn emitted(&self, ray: &Ray, hit: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Color;
}

#[enum_dispatch(Material)]
#[derive(Debug, Clone)]
pub enum MaterialType {
    Isotropic,
    Lambertian,
    Metal,
    Dielectric,
    Diffuse,
}

impl Default for MaterialType {
    fn default() -> MaterialType {
        MaterialType::from(Lambertian {
            albedo: Texture::from(SolidColor {
                color: color(0.0, 1.0, 1.0),
            }),
        })
    }
}
