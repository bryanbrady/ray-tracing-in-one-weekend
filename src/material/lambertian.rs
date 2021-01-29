use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::onb::Onb;
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
        //eprintln!("Lambertian::new(albedo: {:?}", albedo);
        Arc::new(MaterialType::from(Lambertian { albedo: albedo }))
    }
}

impl Material for Lambertian {

    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let uvw = Onb::new(&hit.normal);
        let scatter_direction = uvw.local(&Vec3::random_cosine_direction(rng));
        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction.unit_vector(),
            time: ray.time,
        };
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        let pdf = uvw.w().dot(scattered.direction) / std::f64::consts::PI;
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation,
            pdf: pdf,
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = hit.normal.dot(scattered.direction.unit_vector());
        if cosine < 0.0 { 0.0 } else { cosine / std::f64::consts::PI }
    }

    fn emitted(&self, _ray: &Ray, _hit: &HitRecord, _u: f64, _v: f64, _p: Vec3) -> Color {
        return color(0.0, 0.0, 0.0);
    }
}
