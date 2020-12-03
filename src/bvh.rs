use crate::aabb::Aabb;
use crate::hittable::Hittable;

pub struct BvhNode {
    pub bbox: Aabb,
    pub left: Hittable,
    pub right: Hittable,
}

