use crate::hittable::{aabb::Aabb, hittable_list::HittableList, HitRecord, Hittable, Hittables};
use crate::ray::Ray;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct BvhNode {
    pub bbox: Aabb,
    pub left: Box<Hittables>,
    pub right: Box<Hittables>,
}

impl BvhNode {
    pub fn new(hl: HittableList, time0: f64, time1: f64) -> BvhNode {
        let mut rng = SmallRng::from_entropy();
        let mut objects = hl.hittables.clone();
        let axis = rng.gen::<u32>() % 3;
        objects.sort_by(|a, b| box_compare(a, b, axis));
        return BvhNode::_new(&objects, time0, time1, &mut rng);
    }

    fn _new(objects: &Vec<Hittables>, time0: f64, time1: f64, rng: &mut SmallRng) -> BvhNode {
        //eprintln!("length {:?} :: {:?}\n", objects.len(), objects);
        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => (objects[0].clone(), objects[1].clone()),
            _ => {
                let midpoint = objects.len() / 2;
                (
                    Hittables::from(BvhNode::_new(
                        &objects[..midpoint].to_vec(),
                        time0,
                        time1,
                        rng,
                    )),
                    Hittables::from(BvhNode::_new(
                        &objects[midpoint..].to_vec(),
                        time0,
                        time1,
                        rng,
                    )),
                )
            }
        };

        let bbox = match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (Some(lbox), Some(rbox)) => Aabb::surrounding_box(lbox, rbox),
            _ => panic!("Bounding box doesn't exist"),
        };
        BvhNode {
            bbox: bbox,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rng: &mut SmallRng) -> Option<HitRecord> {
        match self.bbox.hit(ray, t_min, t_max, rng) {
            Some(_) => {
                let hit_left = self.left.hit(ray, t_min, t_max, rng);
                let hit_right = self.right.hit(ray, t_min, t_max, rng);
                match (hit_left, hit_right) {
                    (Some(hit_left), Some(hit_right)) => {
                        if hit_left.t < hit_right.t {
                            Some(hit_left)
                        } else {
                            Some(hit_right)
                        }
                    }
                    (Some(hit_left), None) => Some(hit_left),
                    (None, Some(hit_right)) => Some(hit_right),
                    _ => None,
                }
            }
            None => {
                return None;
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bbox)
    }
}

pub fn box_compare(a: &Hittables, b: &Hittables, axis: u32) -> Ordering {
    // X: axis == 0
    // Y: axis == 1
    // Z: axis == 2
    let cmp = match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(box_a), Some(box_b)) => match axis {
            0 => box_a.minimum.x < box_b.minimum.x,
            1 => box_a.minimum.y < box_b.minimum.y,
            _ => box_a.minimum.z < box_b.minimum.z,
        },
        _ => false,
    };
    if cmp {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
