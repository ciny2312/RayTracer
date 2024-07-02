use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Vec3;
use crate::rtweekend::vec3::Point3;
//use crate::rtweekend::vec3::Color;
use crate::rtweekend::interval::Interval;

pub struct HitRecord{
    pub p:Point3,
    pub normal:Vec3,
    pub t:f64,
    pub front_face:bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        //outward_normal has unit length
        self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t:&Interval) -> (HitRecord, bool);
}