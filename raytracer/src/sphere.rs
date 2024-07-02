use std::sync::Arc;

use crate::hittable_list::hittable::HitRecord;
use crate::hittable_list::hittable::Hittable;
use crate::hittable_list::material::Lambertian;
use crate::hittable_list::material::Material;

use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (HitRecord, bool) {
        let v = Vec3 { e: [0.0, 0.0, 0.0] };
        let mut rec = HitRecord {
            p: v.clone(),
            normal: v.clone(),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian {
                albedo: Color::new(),
            }),
        };
        let oc = self.center - r.ori;
        let a = r.dir.sq_length();
        let h = Vec3::dot(&r.dir, &oc);
        let c = oc.sq_length() - self.radius * self.radius;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();
        (rec, true)
    }
}
