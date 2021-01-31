use crate::camera::Camera;
use crate::hittable::Hittables;

pub mod cornell_box;
pub mod cornell_smoke;
pub mod next_week_final;
pub mod perlin;
pub mod random_world;
pub mod rotate_test;
pub mod simple_light;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub hittables: Hittables,
    pub lights: Hittables,
}
