pub mod hittable;
pub mod material;
pub mod perlin;
pub mod texture;

use crate::aabb::merge;
use crate::aabb::Aabb;
//use crate::aabb::point_to_aabb;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::INF;
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
        bbox: Aabb,
    },
    Quad {
        q: Point3,
        u: Vec3,
        v: Vec3,
        w: Vec3,
        mat: Material,
        bbox: Aabb,
        normal: Vec3,
        d: f64,
    },
    Bvh {
        left: Box<HitObject>,
        right: Box<HitObject>,
        bbox: Aabb,
    },
    HittableList {
        objects: Vec<HitObject>,
        bbox: Aabb,
    },
    Translate {
        object: Box<HitObject>,
        offset: Vec3,
        bbox: Aabb,
    },
    Rotate {
        object: Box<HitObject>,
        sin_theta: f64,
        cos_theta: f64,
        bbox: Aabb,
    },
    ConstantMedium {
        boundary: Box<HitObject>,
        neg_inv_density: f64,
        phase_function: Material,
    },
}

impl HitObject {
    pub fn bounding_box(&self) -> Aabb {
        match self {
            HitObject::Sphere {
                center_st: _,
                radius: _,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox,
            } => bbox.clone(),
            HitObject::Quad {
                q: _,
                u: _,
                v: _,
                w: _,
                mat: _,
                bbox,
                normal: _,
                d: _,
            } => bbox.clone(),
            HitObject::Bvh {
                left: _,
                right: _,
                bbox,
            } => bbox.clone(),
            HitObject::HittableList { objects: _, bbox } => bbox.clone(),
            HitObject::Translate {
                object: _,
                offset: _,
                bbox,
            } => bbox.clone(),
            HitObject::Rotate {
                object: _,
                sin_theta: _,
                cos_theta: _,
                bbox,
            } => bbox.clone(),
            HitObject::ConstantMedium {
                boundary,
                neg_inv_density: _,
                phase_function: _,
            } => boundary.bounding_box(),
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
            HitObject::Quad {
                q: _,
                u: _,
                v: _,
                w: _,
                mat: _,
                bbox: _,
                normal: _,
                d: _,
            } => Vec::new(),
            HitObject::Bvh {
                left: _,
                right: _,
                bbox: _,
            } => Vec::new(),
            HitObject::HittableList { objects, bbox: _ } => objects.clone(),
            HitObject::Translate {
                object: _,
                offset: _,
                bbox: _,
            } => Vec::new(),
            HitObject::Rotate {
                object: _,
                sin_theta: _,
                cos_theta: _,
                bbox: _,
            } => Vec::new(),
            HitObject::ConstantMedium {
                boundary: _,
                neg_inv_density: _,
                phase_function: _,
            } => Vec::new(),
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
            HitObject::Quad {
                q: _,
                u: _,
                v: _,
                w: _,
                mat: _,
                bbox: _,
                normal: _,
                d: _,
            } => Vec3::new(),
            HitObject::Bvh {
                left: _,
                right: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::HittableList {
                objects: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::Translate {
                object: _,
                offset: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::Rotate {
                object: _,
                sin_theta: _,
                cos_theta: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::ConstantMedium {
                boundary: _,
                neg_inv_density: _,
                phase_function: _,
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
                (rec.u, rec.v) = hittable::get_sphere_uv(&outward_normal);
                rec.set_face_normal(r, outward_normal);
                rec.mat = mat.clone();
                (rec, true)
            }
            HitObject::Quad {
                q,
                u,
                v,
                w,
                mat,
                bbox: _,
                normal,
                d,
            } => {
                let mut rec = HitRecord::new();
                let denom = Vec3::dot(normal, &r.dir);
                if denom.abs() < 1e-8 {
                    return (rec, false);
                }
                let t = (d - Vec3::dot(normal, &r.ori)) / denom;
                if !ray_t.contains(t) {
                    return (rec, false);
                }
                let intersection = r.at(t);
                let planar_hitpt_vector = intersection - *q;
                let alpha = Vec3::dot(w, &Vec3::cross(&planar_hitpt_vector, v));
                let beta = Vec3::dot(w, &Vec3::cross(u, &planar_hitpt_vector));
                if !hittable::is_interior(alpha, beta, &mut rec) {
                    return (rec, false);
                }

                rec.t = t;
                rec.p = intersection;
                rec.mat = mat.clone();
                rec.set_face_normal(r, *normal);
                (rec, true)
            }
            HitObject::Bvh { left, right, bbox } => {
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
                (if f2 { hit_right } else { hit_left }, f1 || f2)
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
                (rec, hit_anything)
            }
            HitObject::Translate {
                object,
                offset,
                bbox: _,
            } => {
                let offset_r = Ray {
                    ori: r.ori - *offset,
                    dir: r.dir,
                    tm: r.tm,
                };
                let (rec, flag) = object.hit(&offset_r, ray_t);
                if !flag {
                    return (rec, flag);
                }
                let mut rec_without_offset = rec;
                rec_without_offset.p = rec_without_offset.p + *offset;
                (rec_without_offset, true)
            }
            HitObject::Rotate {
                object,
                sin_theta,
                cos_theta,
                bbox: _,
            } => {
                let mut ori = r.ori;
                let mut dir = r.dir;
                ori.e[0] = cos_theta * r.ori.e[0] - sin_theta * r.ori.e[2];
                ori.e[2] = sin_theta * r.ori.e[0] + cos_theta * r.ori.e[2];
                dir.e[0] = cos_theta * r.dir.e[0] - sin_theta * r.dir.e[2];
                dir.e[2] = sin_theta * r.dir.e[0] + cos_theta * r.dir.e[2];

                let rotated_r = Ray { ori, dir, tm: r.tm };

                let (rec, flag) = object.hit(&rotated_r, ray_t);
                if !flag {
                    return (rec, flag);
                }
                let mut p = rec.p;
                p.e[0] = cos_theta * rec.p.e[0] + sin_theta * rec.p.e[2];
                p.e[2] = -sin_theta * rec.p.e[0] + cos_theta * rec.p.e[2];

                let mut normal = rec.normal;
                normal.e[0] = cos_theta * rec.normal.e[0] + sin_theta * rec.normal.e[2];
                normal.e[2] = -sin_theta * rec.normal.e[0] + cos_theta * rec.normal.e[2];

                let mut rec_cur = rec;
                rec_cur.p = p;
                rec_cur.normal = normal;

                (rec_cur, true)
            }
            HitObject::ConstantMedium {
                boundary,
                neg_inv_density,
                phase_function,
            } => {
                let enable_debug = false;
                let _debugging = enable_debug && random_double_01() < 0.00001;
                let (mut rec1, flag) = boundary.hit(r, &crate::rtweekend::interval::UNIVERSE);
                if !flag {
                    return (rec1, flag);
                }
                let (mut rec2, flag) = boundary.hit(
                    r,
                    &Interval {
                        min: rec1.t + 0.0001,
                        max: INF,
                    },
                );
                if !flag {
                    return (rec2, flag);
                }
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                if rec1.t >= rec2.t {
                    return (rec1, false);
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = neg_inv_density * random_double_01().ln();
                if hit_distance > distance_inside_boundary {
                    return (rec1, false);
                }
                (
                    HitRecord {
                        t: rec1.t + hit_distance / ray_length,
                        p: r.at(rec1.t + hit_distance / ray_length),
                        normal: Vec3 { e: [1.0, 0.0, 0.0] },
                        front_face: true,
                        mat: phase_function.clone(),
                        u: 0.0,
                        v: 0.0,
                    },
                    true,
                )
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
            HitObject::Quad {
                q: _,
                u: _,
                v: _,
                w: _,
                mat: _,
                bbox: _,
                normal: _,
                d: _,
            } => (),
            HitObject::Bvh {
                left: _,
                right: _,
                bbox: _,
            } => (),
            HitObject::HittableList { objects, bbox } => {
                objects.push(object.clone());
                *bbox = merge(bbox, &object.bounding_box());
            }
            HitObject::Translate {
                object: _,
                offset: _,
                bbox: _,
            } => (),
            HitObject::Rotate {
                object: _,
                sin_theta: _,
                cos_theta: _,
                bbox: _,
            } => (),
            HitObject::ConstantMedium {
                boundary: _,
                neg_inv_density: _,
                phase_function: _,
            } => (),
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
