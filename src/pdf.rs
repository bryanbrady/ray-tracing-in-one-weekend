use crate::hittable::{Hittable, Hittables};
use crate::onb::Onb;
use crate::util::random_double;
use crate::vec::Vec3;

use enum_dispatch::enum_dispatch;
use rand::rngs::SmallRng;
use std::sync::Arc;

#[enum_dispatch]
pub trait Pdf {
    fn value(&self, _direction: Vec3, _rng: &mut SmallRng) -> f64 {
        0.0
    }
    fn generate(&self, _rng: &mut SmallRng) -> Vec3 {
        Vec3::zero()
    }
}

#[enum_dispatch(Pdf)]
pub enum PdfType {
    CosinePdf,
    HittablePdf,
    MixturePdf,
}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> PdfType {
        PdfType::from(CosinePdf { uvw: Onb::new(&w) })
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3, _rng: &mut SmallRng) -> f64 {
        let cosine = direction.unit_vector().dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.uvw.local(&random_cosine_direction(rng))
    }
}

pub struct HittablePdf {
    pub origin: Vec3,
    pub object: Arc<Hittables>,
}

impl HittablePdf {
    pub fn new(origin: Vec3, object: Arc<Hittables>) -> PdfType {
        PdfType::from(HittablePdf {
            origin: origin,
            object: object,
        })
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3, rng: &mut SmallRng) -> f64 {
        self.object.pdf_value(self.origin, direction, rng)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        self.object.random(self.origin, rng)
    }
}

pub struct MixturePdf {
    pub pdf1: Arc<PdfType>,
    pub pdf2: Arc<PdfType>,
}

impl MixturePdf {
    pub fn new(pdf1: Arc<PdfType>, pdf2: Arc<PdfType>) -> PdfType {
        PdfType::from(MixturePdf {
            pdf1: pdf1,
            pdf2: pdf2,
        })
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3, rng: &mut SmallRng) -> f64 {
        0.5 * self.pdf1.value(direction, rng) + 0.5 * self.pdf2.value(direction, rng)
    }

    fn generate(&self, rng: &mut SmallRng) -> Vec3 {
        if random_double(rng) < 0.5 {
            self.pdf1.generate(rng)
        } else {
            self.pdf2.generate(rng)
        }
    }
}

fn random_cosine_direction(rng: &mut SmallRng) -> Vec3 {
    let r1 = random_double(rng);
    let r2 = random_double(rng);
    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = f64::cos(phi) * f64::sqrt(r2);
    let y = f64::sin(phi) * f64::sqrt(r2);
    let z = f64::sqrt(1.0 - r2);
    Vec3::new(x, y, z)
}
