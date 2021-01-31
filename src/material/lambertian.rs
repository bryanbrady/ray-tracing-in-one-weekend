use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::onb::Onb;
use crate::pdf::CosinePdf;
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

// Lambertian
#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Arc<MaterialType> {
        Arc::new(MaterialType::from(Lambertian { albedo: albedo }))
    }
}

impl Material for Lambertian {

    fn scatter(&self, rayin: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let uvw = Onb::new(&hit.normal);
        let scatter_direction = uvw.local(&Vec3::random_cosine_direction(rng));
        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction.unit_vector(),
            time: rayin.time,
        };
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some(Scatter {
            ray: scattered,
            attenuation: attenuation,
            pdf: Some(CosinePdf::new(hit.normal))
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = hit.normal.dot(scattered.direction.unit_vector());
        if cosine < 0.0 { 0.0 } else { cosine / std::f64::consts::PI }
    }
}
