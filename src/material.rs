use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material{
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray { origin: hit.point, direction: scatter_direction };
        let attenuation = self.albedo;
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}


impl Material for Metal {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = Vec3::reflect(_ray.direction.unit_vector(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere()
        };
        let attenuation = self.albedo;
        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some(Scatter {scattered: scattered, attenuation: attenuation})
        }
        return None
    }
}
