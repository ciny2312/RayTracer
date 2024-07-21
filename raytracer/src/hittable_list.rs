pub mod hittable;
pub mod material;
pub mod perlin;
pub mod texture;

use crate::onb::Onb;

use crate::aabb::merge;
use crate::aabb::Aabb;

//use crate::aabb::point_to_aabb;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double_01;
use crate::rtweekend::random_int;
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
        area: f64,
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
    Triangle {
        v1:Point3,
        v2:Point3,
        v3:Point3,
        mat:Material,
        normal:Vec3,
        bbox:Aabb,
        area:f64,
    }
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
                area: _,
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
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox,
                area:_,
            }=>bbox.clone(),
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
                area: _,
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
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox:_,
                area:_,
            }=>Vec::new()
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
                area: _,
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
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox:_,
                area:_,
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
                area: _,
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
            HitObject::Triangle { v1, v2, v3, mat,normal,bbox,
                area:_, }=>{
                let mut rec = HitRecord::new();
                if Vec3::dot(&Vec3::unit_vector(r.ori-*v1),normal)<=0.0001{
                    return (rec,false);
                }
                let light_d=Vec3::dot(&(*v1-r.ori),normal)/Vec3::dot(&r.dir,normal);
                if light_d<=0.0{
                    return (rec,false);
                }
                rec.t = light_d;
                rec.p = r.at(light_d);
                rec.mat = mat.clone();
                rec.set_face_normal(r, *normal);
 
                let a=Vec3::unit_vector(*v1-rec.p);
                let b=Vec3::unit_vector(*v2-rec.p);
                let c=Vec3::unit_vector(*v3-rec.p);
                let sa=Vec3::unit_vector(Vec3::cross(&a,& b));
                let sb=Vec3::unit_vector(Vec3::cross(&b,& c));
                let sc=Vec3::unit_vector(Vec3::cross(&c,& a));

                if !(Vec3::dot(&sa,&sb) >0.999 && Vec3::dot(&sb,&sc) >0.999 && Vec3::dot(&sc,&sa) >0.999){
                    return (rec,false);
                }
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
                area: _,
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
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox:_,
                area:_,
            }=>(),
        }
    }
    pub fn pdf_value(&self, ori: Point3, dir: Vec3) -> f64 {
        match self {
            HitObject::Sphere {
                center_st,
                radius,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox: _,
            } => {
                let (_rec, flag) = self.hit(
                    &Ray { ori, dir, tm: 0.0 },
                    &Interval {
                        min: 0.001,
                        max: INF,
                    },
                );
                if !flag {
                    return 0.0;
                }
                let cos_theta_max = (1.0 - radius * radius / (*center_st - ori).sq_length()).sqrt();
                let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            HitObject::Quad {
                q: _,
                u: _,
                v: _,
                w: _,
                mat: _,
                bbox: _,
                normal: _,
                d: _,
                area,
            } => {
                let (rec, flag) = self.hit(
                    &Ray { ori, dir, tm: 0.0 },
                    &Interval {
                        min: 0.001,
                        max: INF,
                    },
                );
                if !flag {
                    return 0.0;
                }
                let distance_squared = rec.t * rec.t * dir.sq_length();
                let cosine = (Vec3::dot(&dir, &rec.normal) / dir.length()).abs();
                distance_squared / (cosine * area)
            }
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox:_,
                area,
            }=>{
                1.0/area
            }
            HitObject::Bvh {
                left: _,
                right: _,
                bbox: _,
            } => 0.0,
            HitObject::HittableList { objects, bbox: _ } => {
                let weight = 1.0 / objects.len() as f64;
                let mut sum = 0.0;
                for object in objects {
                    //& or not
                    sum += object.pdf_value(ori, dir) * weight;
                }
                sum
            }
            HitObject::Translate {
                object: _,
                offset: _,
                bbox: _,
            } => 0.0,
            HitObject::Rotate {
                object: _,
                sin_theta: _,
                cos_theta: _,
                bbox: _,
            } => 0.0,
            HitObject::ConstantMedium {
                boundary: _,
                neg_inv_density: _,
                phase_function: _,
            } => 0.0,
        }
    }
    pub fn random_from(&self, ori: Point3) -> Vec3 {
        match self {
            HitObject::Sphere {
                center_st,
                radius,
                mat: _,
                is_moving: _,
                center_vec: _,
                bbox: _,
            } => {
                let dir = *center_st - ori;
                let distance_squared = dir.sq_length();
                let uvw = Onb::build_from_w(dir);
                uvw.local(&hittable::random_to_sphere(*radius, distance_squared))
            }
            HitObject::Quad {
                q,
                u,
                v,
                w: _,
                mat: _,
                bbox: _,
                normal: _,
                d: _,
                area: _,
            } => {
                let p = (*q) + ((*u) * random_double_01()) + ((*v) * random_double_01());
                p - ori
            }
            HitObject::Triangle {
                v1:_,
                v2:_,
                v3:_,
                normal:_,
                mat:_,
                bbox:_,
                area:_,
            }=>{
                Vec3::new()
            }
            HitObject::Bvh {
                left: _,
                right: _,
                bbox: _,
            } => Vec3::new(),
            HitObject::HittableList { objects, bbox: _ } => {
                let int_size = objects.len() as i32;
                objects[random_int(0, int_size - 1) as usize].random_from(ori)
            }
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

    /*    pub fn clone(&self) -> HitObject {
        match self{
            HitObject::_Sphere {
                center_st,
                radius ,
                mat,
                is_moving ,
                center_vec,
                bbox ,
            } =>{
                HitObject::_Sphere {
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
