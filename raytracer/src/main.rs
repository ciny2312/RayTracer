use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
struct Vec3 {
    e: [f64; 3],
}
impl Vec3 {
    fn x(&self) -> f64 {
        return self.e[0];
    }
    fn y(&self) -> f64 {
        return self.e[1];
    }
    fn z(&self) -> f64 {
        return self.e[2];
    }
    fn fushu(&self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
    fn sq_length(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }
    fn length(&self) -> f64 {
        return self.sq_length().sqrt();
    }
    fn self_add(&mut self, x: Self) {
        self.e[0] += x.e[0];
        self.e[1] += x.e[1];
        self.e[2] += x.e[2];
    }
    fn self_mul(&mut self, x: f64) {
        self.e[0] *= x;
        self.e[1] *= x;
        self.e[2] *= x;
    }
    fn self_div(&mut self, x: f64) {
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
    fn cross(x: &Self, y: &Self) -> Self {
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
        return Self::div(x, y);
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
    fn _at(&self, t: f64) -> Point3 {
        return Vec3::add(&self.ori, &Vec3::mul(&self.dir, t));
    }
    fn _clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            ori: self.ori.clone(),
        }
    }
}
fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(&r.dir);
    let a = 0.5 * (unit_direction.e[1] + 1.0);
    return Vec3::add(
        &Vec3::mul(&Color { e: [1.0, 1.0, 1.0] }, 1.0 - a),
        &Vec3::mul(&Color { e: [0.5, 0.7, 1.0] }, a),
    );
}
fn main() {
    let path = Path::new("output/book1/image2.ppm");
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
                    e: [0.0, 0.0, -focal_length],
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
            let pixel_color = ray_color(&r);
            Vec3::output(&pixel_color, &mut file);
        }
    }
}
