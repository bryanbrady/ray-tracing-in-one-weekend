use crate::hittable::{
    aabb::Aabb, hittable_list::HittableList, rect::XyRect, rect::XzRect, rect::YzRect, HitRecord,
    Hittable, Hittables,
};
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Box3D {
    pub box_min: Vec3,
    pub box_max: Vec3,
    pub sides: HittableList,
}

#[allow(dead_code)]
impl Box3D {
    pub fn new(p0: Vec3, p1: Vec3, mat: Arc<MaterialType>) -> Hittables {
        let mut sides = HittableList {
            hittables: Vec::new(),
        };
        sides.add(XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, mat.clone()));
        sides.add(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, mat.clone()));
        sides.add(XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, mat.clone()));
        sides.add(XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, mat.clone()));
        sides.add(YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, mat.clone()));
        sides.add(YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, mat.clone()));
        Hittables::from(Box3D {
            box_min: p0,
            box_max: p1,
            sides: sides,
        })
    }
}

impl Hittable for Box3D {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}
