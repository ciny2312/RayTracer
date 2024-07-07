use std::cmp::Ordering;

//use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::hittable_list::material::Lambertian;
use crate::aabb::Aabb;
use crate::hittable_list::material::Material;
use crate::hittable_list::texture::Texture::SolidColor;
use crate::hittable_list::HitObject;
//use crate::hittable_list::texture::Texture::CheckerTexture;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Material,
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
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat: Material::Lambertian {
                tex: Box::new(SolidColor {
                    albedo: Color::new(),
                }),
            },
        }
    }
}

pub fn new_hittable_list() -> HitObject {
    HitObject::HittableList {
        objects: Vec::new(),
        bbox: crate::aabb::EMPTY,
    }
}

fn box_x_compare(a: &HitObject, b: &HitObject) -> Ordering {
    a.bounding_box().b[0]
        .min
        .partial_cmp(&b.bounding_box().b[0].min)
        .unwrap_or(Ordering::Equal)
}
fn box_y_compare(a: &HitObject, b: &HitObject) -> Ordering {
    a.bounding_box().b[1]
        .min
        .partial_cmp(&b.bounding_box().b[1].min)
        .unwrap_or(Ordering::Equal)
}
fn box_z_compare(a: &HitObject, b: &HitObject) -> Ordering {
    a.bounding_box().b[2]
        .min
        .partial_cmp(&b.bounding_box().b[2].min)
        .unwrap_or(Ordering::Equal)
}
pub fn bvh_node(objects: &mut Vec<HitObject>, start: usize, end: usize) -> HitObject {
    let mut bbox = crate::aabb::EMPTY;
    for object_index in objects.iter().take(end).skip(start) {
        bbox = Aabb::merge(&bbox, &object_index.bounding_box());
    }
    let axis = bbox.longest_axis();

    let object_span = end - start;
    let left: HitObject;
    let right: HitObject;

    if object_span == 1 {
        left = objects[start].clone();
        right = objects[start].clone();
    } else if object_span == 2 {
        left = objects[start].clone();
        right = objects[start + 1].clone();
    } else {
        if axis == 0 {
            objects[start..end].sort_by(box_x_compare);
        } else if axis == 1 {
            objects[start..end].sort_by(box_y_compare);
        } else {
            objects[start..end].sort_by(box_z_compare);
        }
        let mid = start + object_span / 2;
        left = bvh_node(objects, start, mid);
        right = bvh_node(objects, mid, end);
    }
    HitObject::Bvh {
        left: Box::new(left),
        right: Box::new(right),
        bbox,
    }
}
pub fn build_sphere(
    center_st: Point3,
    center_vec: Vec3,
    radius: f64,
    mat: Material,
    is_moving: bool,
) -> HitObject {
    let v = Vec3 {
        e: [radius, radius, radius],
    };
    let bbox = if is_moving {
        Aabb::merge(
            &Aabb::point_to_aabb(&(center_st - v), &(center_st + v)),
            &Aabb::point_to_aabb(&(center_st + center_vec - v), &(center_st + center_vec + v)),
        )
    } else {
        Aabb::point_to_aabb(&(center_st - v), &(center_st + v))
    };
    HitObject::Sphere {
        center_st,
        radius,
        mat,
        is_moving,
        center_vec,
        bbox,
    }
}
pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
    let theta = (-p.e[1]).acos();
    let phi = (-p.e[2]).atan2(p.e[0]) + std::f64::consts::PI;
    (
        phi / (2.0 * std::f64::consts::PI),
        theta / std::f64::consts::PI,
    )
}
