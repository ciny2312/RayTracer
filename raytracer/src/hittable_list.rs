pub mod hittable;
use hittable::HitRecord;
use hittable::Hittable;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Vec3;
//use crate::rtweekend::vec3::Point3;
//use crate::rtweekend::vec3::Color;
use crate::rtweekend::ray::Ray;

use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t:&Interval) -> (HitRecord, bool) {
        let v = Vec3::new();
        let mut rec = HitRecord {
            p: v.clone(),
            normal: v.clone(),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let (temp_rec, flag) = object.hit(r, &Interval{min:ray_t.min, max:closest_so_far});
            if flag {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        (rec, hit_anything)
    }
}
impl HittableList {
    pub fn _clear(&mut self) {
        self.objects = Vec::new();
    }
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}