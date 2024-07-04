pub mod hittable;
pub mod material;

use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
use hittable::HitRecord;
//use hittable::Hittable;
//use crate::rtweekend::vec3::Point3;
use crate::hittable_list::material::Material;

#[derive(Clone)]
pub enum HitObject {
    Sphere {
        center: Point3,
        radius: f64,
        mat: Material,
    },
}
impl HitObject {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (HitRecord, bool) {
        match *self {
            HitObject::Sphere {
                center,
                radius,
                mat,
            } => {
                let v = Vec3 { e: [0.0, 0.0, 0.0] };
                let mut rec = HitRecord {
                    p: v.clone(),
                    normal: v.clone(),
                    t: 0.0,
                    front_face: false,
                    mat: Material::Lambertian {
                        albedo: Color::new(),
                    },
                };
                let oc = center - r.ori;
                let a = r.dir.sq_length();
                let h = Vec3::dot(&r.dir, &oc);
                let c = oc.sq_length() - radius * radius;
                let discriminant = h * h - a * c;
                if discriminant < 0.0 {
                    return (rec, false);
                }
                let sqrtd = discriminant.sqrt();
                let mut root = (h - sqrtd) / a;
                if !ray_t.surrounds(root) {
                    root = (h + sqrtd) / a;
                    if !ray_t.surrounds(root) {
                        return (rec, false);
                    }
                }
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - center) / radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = mat.clone();
                (rec, true)
            }
        }
    }
}

pub struct HittableList {
    objects: Vec<HitObject>,
}
impl HittableList {
    pub fn clone(&self) -> Self {
        HittableList {
            objects: self.objects.clone(),
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> (HitRecord, bool) {
        let mut rec = HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            mat: Material::Lambertian {
                albedo: Color::new(),
            },
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

    pub fn _clear(&mut self) {
        self.objects = Vec::new();
    }
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: HitObject) {
        self.objects.push(object);
    }
}
