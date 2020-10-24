
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::util::*;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3,
               lookat: Vec3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64)
               -> Camera {

        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Camera {
            origin: origin,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray{
            origin: self.origin,
            direction: &self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin
        }
    }
}


