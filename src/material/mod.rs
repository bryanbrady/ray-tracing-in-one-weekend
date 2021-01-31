use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::material::{
    dielectric::Dielectric, diffuse::Diffuse, isotropic::Isotropic, lambertian::Lambertian,
    metal::Metal,
};
use crate::pdf::PdfType;
use crate::ray::Ray;
use crate::texture::{solidcolor::SolidColor, Texture};
use crate::vec::Vec3;
use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;

pub mod dielectric;
pub mod diffuse;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
    pub pdf: Option<PdfType>,
}

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, _rayin: &Ray, _hit: &HitRecord, _rng: &mut SmallRng) -> Option<Scatter> {
        None
    }
    fn scattering_pdf(&self, _rayin: &Ray, _hit: &HitRecord, _scattered: &Ray) -> f64 {
        1.0
    }
    fn emitted(&self, _rayin: &Ray, _hit: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Color {
        color(0.0, 0.0, 0.0)
    }
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
