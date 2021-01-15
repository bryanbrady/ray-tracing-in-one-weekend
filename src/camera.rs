use crate::ray::Ray;
use crate::util::*;
use crate::vec::Vec3;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = lookfrom;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut SmallRng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: &self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time: if self.time0 != self.time1 {
                rng.gen_range(self.time0, self.time1)
            } else {
                self.time0
            },
        }
    }
}
