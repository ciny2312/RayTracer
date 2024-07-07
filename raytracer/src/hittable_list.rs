pub mod hittable;
pub mod material;
pub mod texture;

use crate::aabb::AABB;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Point3;
//use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Vec3;
use hittable::HitRecord;
//use hittable::Hittable;
use crate::hittable_list::material::Material;

#[derive(Clone, Debug)]
pub enum HitObject {
    Sphere {
        center_st: Point3,
        radius: f64,
        mat: Material,
        is_moving: bool,
        center_vec: Vec3,
        bbox: AABB,
    },
    BVH {
        left: Box<HitObject>,
        right: Box<HitObject>,
        bbox: AABB,
    },
    HittableList {
        objects: Vec<HitObject>,
        bbox: AABB,
    },
}

impl HitObject {
    pub fn bounding_box(&self) -> AABB {
        match self {
            HitObject::Sphere {
                center_st: _,
                radius: _,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox,
            } => bbox.clone(),
            HitObject::BVH {
                left: _,
                right: _,
                bbox,
            } => bbox.clone(),
            HitObject::HittableList { objects: _, bbox } => bbox.clone(),
        }
    }
    pub fn get_objects(&self) -> Vec<HitObject> {
        match self {
            HitObject::Sphere {
                center_st: _,
                radius: _,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox: _,
            } => Vec::new(),
            HitObject::BVH {
                left: _,
                right: _,
                bbox: _,
            } => Vec::new(),
            HitObject::HittableList { objects, bbox: _ } => objects.clone(),
        }
    }
    fn cur_center(&self, time: f64) -> Point3 {
        match self {
            HitObject::Sphere {
                center_st,
                radius: _,
                mat: _,
                is_moving: _,
                center_vec,
                bbox: _,
            } => *center_st + (*center_vec) * time,
            HitObject::BVH {
                left: _,
                right: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::HittableList {
                objects: _,
                bbox: _,
            } => Vec3::new(),
        }
    }
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> (HitRecord, bool) {
        match self {
            HitObject::Sphere {
                center_st,
                radius,
                mat,
                is_moving,
                center_vec: _,
                bbox: _,
            } => {
                let mut rec = HitRecord::new();
                let center = if *is_moving {
                    self.cur_center(r.tm)
                } else {
                    *center_st
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
                let outward_normal = (rec.p - center) / *radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = mat.clone();
                (rec, true)
            }
            HitObject::BVH { left, right, bbox } => {
                if !bbox.hit(r, ray_t) {
                    return (HitRecord::new(), false);
                }
                let (hit_left, f1) = left.hit(r, ray_t);
                let (hit_right, f2) = right.hit(
                    r,
                    &Interval {
                        min: ray_t.min,
                        max: if f1 { hit_left.t } else { ray_t.max },
                    },
                );
                return (if f2 { hit_right } else { hit_left }, f1 || f2);
            }
            HitObject::HittableList {
                objects: _,
                bbox: _,
            } => {
                let mut rec = HitRecord::new();
                let mut hit_anything = false;
                let mut closest_so_far = ray_t.max;

                for object in &self.get_objects() {
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
                return (rec, hit_anything);
            }
        }
    }
    pub fn add(&mut self, object: HitObject) {
        match self {
            HitObject::Sphere {
                center_st: _,
                radius: _,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox: _,
            } => (),
            HitObject::BVH {
                left: _,
                right: _,
                bbox: _,
            } => (),
            HitObject::HittableList { objects, bbox } => {
                objects.push(object.clone());
                *bbox = AABB::merge(bbox, &object.bounding_box());
            }
        }
    }
    /*    pub fn clone(&self) -> HitObject {
        match self{
            HitObject::Sphere {
                center_st,
                radius ,
                mat,
                is_moving ,
                center_vec,
                bbox ,
            } =>{
                HitObject::Sphere {
                    center_st:*center_st,
                    radius:*radius,
                    mat:*mat,
                    is_moving:*is_moving,
                    center_vec:*center_vec,
                    bbox:bbox.clone() ,
                }
            }
            HitObject::BVH { left, right, bbox } => {
                HitObject::BVH{
                    left:left.clone(),
                    right:right.clone(),
                    bbox:bbox.clone()
                }
            }
            HitObject::HittableList{
                objects,
                bbox,
            }=>{
                HitObject::HittableList {
                    objects:objects.clone(),
                    bbox:bbox.clone(),
                }
            }
        }
    }
    */
}
