use std::boxed::Box;
use std::vec::Vec;

use super::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

// Should this be a hittable?? It can be
// It feels like this could create some weird reference loops that could fuck 
// some shit up.
impl Hittable for HittableList {

    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec;
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }
}

