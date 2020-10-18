
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, viewport_width: f64, focal_length: f64) -> Camera {
        let origin = Vec3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            aspect_ratio: aspect_ratio,
            viewport_height: viewport_height,
            viewport_width: viewport_width,
            focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length}
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let r = Ray{
            origin: self.origin,
            direction: &self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
        };
        return r
    }
}


