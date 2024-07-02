pub mod hittable;
pub mod material;

use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Vec3;
use hittable::HitRecord;
use hittable::Hittable;
//use crate::rtweekend::vec3::Point3;
use crate::hittable_list::material::Lambertian;

use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (HitRecord, bool) {
        let mut rec = HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian {
                albedo: Color::new(),
            }),
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let (temp_rec, flag) = object.hit(
                r,
                &Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            );
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
