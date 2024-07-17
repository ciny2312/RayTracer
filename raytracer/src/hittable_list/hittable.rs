use std::cmp::Ordering;

//use crate::rtweekend::interval::Interval;
use crate::rtweekend::degrees_to_radians;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
use crate::rtweekend::INF;
//use crate::hittable_list::material::Lambertian;
//use crate::aabb::Aabb;
use crate::aabb::merge;
use crate::aabb::point_to_aabb;
use crate::hittable_list::material::Material;
use crate::hittable_list::texture::Texture;
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
                tex: Box::new(Texture::SolidColor {
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
        bbox = merge(&bbox, &object_index.bounding_box());
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
        merge(
            &point_to_aabb(&(center_st - v), &(center_st + v)),
            &point_to_aabb(&(center_st + center_vec - v), &(center_st + center_vec + v)),
        )
    } else {
        point_to_aabb(&(center_st - v), &(center_st + v))
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
pub fn build_quad(q: Point3, u: Vec3, v: Vec3, mat: Material) -> HitObject {
    let n = Vec3::cross(&u, &v);
    let normal = Vec3::unit_vector(n);
    let d = Vec3::dot(&normal, &q);
    let w = n / Vec3::dot(&n, &n);
    let bbox = merge(
        &point_to_aabb(&q, &(q + u + v)),
        &point_to_aabb(&(q + u), &(q + v)),
    );
    let area = n.length();
    HitObject::Quad {
        q,
        u,
        v,
        w,
        mat,
        normal,
        bbox,
        d,
        area,
    }
}
pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    let unit_interval = Interval { min: 0.0, max: 1.0 };
    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }
    rec.u = a;
    rec.v = b;
    true
}
pub fn build_box(a: &Point3, b: &Point3, mat: &Material) -> HitObject {
    let mut sides = new_hittable_list();
    let min = Point3 {
        e: [a.e[0].min(b.e[0]), a.e[1].min(b.e[1]), a.e[2].min(b.e[2])],
    };
    let max = Point3 {
        e: [a.e[0].max(b.e[0]), a.e[1].max(b.e[1]), a.e[2].max(b.e[2])],
    };
    let dx = Vec3 {
        e: [max.e[0] - min.e[0], 0.0, 0.0],
    };
    let dy = Vec3 {
        e: [0.0, max.e[1] - min.e[1], 0.0],
    };
    let dz = Vec3 {
        e: [0.0, 0.0, max.e[2] - min.e[2]],
    };
    sides.add(build_quad(
        Point3 {
            e: [min.e[0], min.e[1], max.e[2]],
        },
        dx,
        dy,
        mat.clone(),
    ));
    sides.add(build_quad(
        Point3 {
            e: [max.e[0], min.e[1], max.e[2]],
        },
        -dz,
        dy,
        mat.clone(),
    ));
    sides.add(build_quad(
        Point3 {
            e: [max.e[0], min.e[1], min.e[2]],
        },
        -dx,
        dy,
        mat.clone(),
    ));
    sides.add(build_quad(
        Point3 {
            e: [min.e[0], min.e[1], min.e[2]],
        },
        dz,
        dy,
        mat.clone(),
    ));
    sides.add(build_quad(
        Point3 {
            e: [min.e[0], max.e[1], max.e[2]],
        },
        dx,
        -dz,
        mat.clone(),
    ));
    sides.add(build_quad(
        Point3 {
            e: [min.e[0], min.e[1], min.e[2]],
        },
        dx,
        dz,
        mat.clone(),
    ));
    sides
}
pub fn build_translate(object: &HitObject, offset: Vec3) -> HitObject {
    let pre = object.bounding_box();
    let mut bbox = crate::aabb::EMPTY;
    for i in 0..3 {
        bbox.b[i] = Interval {
            min: pre.b[i].min + offset.e[i],
            max: pre.b[i].max + offset.e[i],
        };
    }
    HitObject::Translate {
        object: Box::new(object.clone()),
        offset,
        bbox,
    }
}
pub fn build_rotate(object: &HitObject, angle: f64) -> HitObject {
    let radians = degrees_to_radians(angle);
    let sin_theta = radians.sin();
    let cos_theta = radians.cos();
    let bbox = object.bounding_box();

    let mut min = Point3 { e: [INF, INF, INF] };
    let mut max = Point3 {
        e: [-INF, -INF, -INF],
    };

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let x = i as f64 * bbox.b[0].max + (1 - i) as f64 * bbox.b[0].min;
                let y = j as f64 * bbox.b[1].max + (1 - j) as f64 * bbox.b[1].min;
                let z = k as f64 * bbox.b[2].max + (1 - k) as f64 * bbox.b[2].min;

                let newx = cos_theta * x + sin_theta * z;
                let newz = -sin_theta * x + cos_theta * z;

                let tester = Vec3 { e: [newx, y, newz] };
                for c in 0..3 {
                    min.e[c] = min.e[c].min(tester.e[c]);
                    max.e[c] = max.e[c].max(tester.e[c]);
                }
            }
        }
    }
    HitObject::Rotate {
        object: Box::new(object.clone()),
        sin_theta,
        cos_theta,
        bbox: point_to_aabb(&min, &max),
    }
}
pub fn build_constant_medium(boundary: &HitObject, density: f64, tex: &Texture) -> HitObject {
    HitObject::ConstantMedium {
        boundary: Box::new(boundary.clone()),
        neg_inv_density: -1.0 / density,
        phase_function: Material::Isotropic {
            tex: Box::new(tex.clone()),
        },
    }
}
pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double_01();
    let r2 = random_double_01();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * std::f64::consts::PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3 { e: [x, y, z] }
}
