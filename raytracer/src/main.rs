use std::fs::{self, File};
use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::path::Path;
use std::sync::Arc;
//mod rtweekend;
//mod HittableList;
//mod sphere;

const _PI: f64 = 3.1415926535897932385;
const INF: f64 = 1e18;
fn _degree_to_radians(degree: f64) -> f64 {
    degree * _PI / 180.0
}

struct Vec3 {
    e: [f64; 3],
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl Vec3 {
    fn _x(&self) -> f64 {
        self.e[0]
    }
    fn _y(&self) -> f64 {
        self.e[1]
    }
    fn _z(&self) -> f64 {
        self.e[2]
    }
    fn fushu(&self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
    fn sq_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    fn length(&self) -> f64 {
        self.sq_length().sqrt()
    }
    fn _self_add(&mut self, x: Self) {
        self.e[0] += x.e[0];
        self.e[1] += x.e[1];
        self.e[2] += x.e[2];
    }
    fn _self_mul(&mut self, x: f64) {
        self.e[0] *= x;
        self.e[1] *= x;
        self.e[2] *= x;
    }
    fn _self_div(&mut self, x: f64) {
        self.e[0] /= x;
        self.e[1] /= x;
        self.e[2] /= x;
    }
    fn add(x: &Self, y: &Self) -> Self {
        Self {
            e: [x.e[0] + y.e[0], x.e[1] + y.e[1], x.e[2] + y.e[2]],
        }
    }
    fn del(x: &Self, y: &Self) -> Self {
        Self {
            e: [x.e[0] - y.e[0], x.e[1] - y.e[1], x.e[2] - y.e[2]],
        }
    }
    fn mul(x: &Self, y: f64) -> Self {
        Self {
            e: [x.e[0] * y, x.e[1] * y, x.e[2] * y],
        }
    }
    fn div(x: &Self, y: f64) -> Self {
        Self {
            e: [x.e[0] / y, x.e[1] / y, x.e[2] / y],
        }
    }
    fn dot(x: &Self, y: &Self) -> f64 {
        x.e[0] * y.e[0] + x.e[1] * y.e[1] + x.e[2] * y.e[2]
    }
    fn _cross(x: &Self, y: &Self) -> Self {
        Self {
            e: [
                x.e[1] * y.e[2] - x.e[2] * y.e[1],
                x.e[2] * y.e[0] - x.e[0] * y.e[2],
                x.e[0] * y.e[1] - x.e[1] * y.e[0],
            ],
        }
    }
    fn unit_vector(x: &Self) -> Self {
        let y = x.length();
        Self::div(x, y)
    }
    fn output(x: &Self, file: &mut File) {
        let r: u32 = (x.e[0] * 255.999) as u32;
        let g: u32 = (x.e[1] * 255.999) as u32;
        let b: u32 = (x.e[2] * 255.999) as u32;
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
    fn clone(&self) -> Self {
        Self {
            e: [self.e[0], self.e[1], self.e[2]],
        }
    }
}
type Color = Vec3;
type Point3 = Vec3;
struct Ray {
    dir: Vec3,
    ori: Point3,
}
impl Ray {
    fn at(&self, t: f64) -> Point3 {
        Vec3::add(&self.ori, &Vec3::mul(&self.dir, t))
    }
    fn _clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            ori: self.ori.clone(),
        }
    }
}
struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}
impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        //outward_normal has unit length
        self.front_face = Vec3::dot(&r.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.fushu()
        };
    }
}
trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> (HitRecord, bool);
}
struct Sphere {
    center: Point3,
    radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> (HitRecord, bool) {
        let v = Vec3 { e: [0.0, 0.0, 0.0] };
        let mut rec = HitRecord {
            p: v.clone(),
            normal: v.clone(),
            t: 0.0,
            front_face: false,
        };
        let oc = Vec3::del(&self.center, &r.ori);
        let a = r.dir.sq_length();
        let h = Vec3::dot(&r.dir, &oc);
        let c = oc.sq_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return (rec, false);
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return (rec, false);
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = Vec3::div(&Vec3::del(&rec.p, &self.center), self.radius);
        rec.set_face_normal(&r, outward_normal);
        (rec, true)
    }
}
struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> (HitRecord, bool) {
        let v = Vec3 { e: [0.0, 0.0, 0.0] };
        let mut rec = HitRecord {
            p: v.clone(),
            normal: v.clone(),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            let (temp_rec, flag) = object.hit(r, ray_tmin, closest_so_far);
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
    fn _clear(&mut self) {
        self.objects = Vec::new();
    }
    fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}
fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let (rec, flag) = world.hit(r, 0.0, INF);
    if flag {
        return (rec.normal + Color { e: [1.0, 1.0, 1.0] }) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(&r.dir);
    let a = 0.5 * (unit_direction.e[1] + 1.0);
    Vec3::add(
        &Vec3::mul(&Color { e: [1.0, 1.0, 1.0] }, 1.0 - a),
        &Vec3::mul(&Color { e: [0.5, 0.7, 1.0] }, a),
    )
}
fn main() {
    let path = Path::new("output/book1/image5.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();

    let aspect_ratio = 16.0 / 9.0;
    let width: f64 = 400.0;
    let height: u32 = (width / aspect_ratio) as u32;
    let width: u32 = 400;
    let height = {
        if height < 1 {
            1
        } else {
            height
        }
    };
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);
    let camera_center = Point3 { e: [0.0, 0.0, 0.0] };
    let viewport_u = Vec3 {
        e: [viewport_width, 0.0, 0.0],
    };
    let viewport_v = Vec3 {
        e: [0.0, -viewport_height, 0.0],
    };
    let delta_u = Vec3::div(&viewport_u, width as f64);
    let delta_v = Vec3::div(&viewport_v, height as f64);
    let viewport_upleft = Vec3::del(
        &Vec3::del(
            &Vec3::del(
                &camera_center,
                &Vec3 {
                    e: [0.0, 0.0, focal_length],
                },
            ),
            &Vec3::div(&viewport_u, 2.0),
        ),
        &Vec3::div(&viewport_v, 2.0),
    );
    let pixel_loc = Vec3::add(
        &viewport_upleft,
        &Vec3::div(&Vec3::add(&delta_u, &delta_v), 2.0),
    );

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
    }));
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
    }));

    writeln!(file, "P3\n{} {}\n255", width, height).unwrap();
    for j in 0..height {
        for i in 0..width {
            let pixel_center = Vec3::add(
                &pixel_loc,
                &Vec3::add(
                    &Vec3::mul(&delta_u, i as f64),
                    &Vec3::mul(&delta_v, j as f64),
                ),
            );
            let ray_direction = Vec3::del(&pixel_center, &camera_center);
            let r = Ray {
                ori: camera_center.clone(),
                dir: ray_direction.clone(),
            };
            let pixel_color = ray_color(&r, &world);
            Vec3::output(&pixel_color, &mut file);
        }
    }
}
