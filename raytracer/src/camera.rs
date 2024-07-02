use std::fs::File;
use std::io::Write;

use crate::hittable_list::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::rtweekend::color::write_color;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
use crate::rtweekend::INF;

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub width: u32,        // Rendered image width in pixel count
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub height: u32,           // Rendered image height
    pub camera_center: Point3, // Camera center
    pub pixel_loc: Point3,     // Location of pixel 0, 0
    pub delta_u: Vec3,         // Offset to pixel to the right
    pub delta_v: Vec3,         // Offset to pixel below
    pub pixel_samples_scale: f64,
}
impl Camera {
    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new();
        }
        let (rec, flag) = world.hit(
            r,
            &Interval {
                min: 0.001,
                max: INF,
            },
        );
        if flag {
            let direction = rec.normal + Vec3::random_unit_vector();
            return Self::ray_color(
                &Ray {
                    ori: rec.p,
                    dir: direction,
                },
                depth - 1,
                world,
            ) * 0.5;
        }
        let unit_direction = Vec3::unit_vector(r.dir);
        let a = 0.5 * (unit_direction.e[1] + 1.0);
        Color { e: [1.0, 1.0, 1.0] } * (1.0 - a) + Color { e: [0.5, 0.7, 1.0] } * a
    }
    fn initialize(&mut self) {
        self.height = (self.width as f64 / self.aspect_ratio) as u32;
        self.height = {
            if self.height < 1 {
                1
            } else {
                self.height
            }
        };
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);
        self.camera_center = Point3 { e: [0.0, 0.0, 0.0] };
        let viewport_u = Vec3 {
            e: [viewport_width, 0.0, 0.0],
        };
        let viewport_v = Vec3 {
            e: [0.0, -viewport_height, 0.0],
        };
        self.delta_u = viewport_u / (self.width as f64);
        self.delta_v = viewport_v / (self.height as f64);
        let viewport_upleft = self.camera_center
            - Vec3 {
                e: [0.0, 0.0, focal_length],
            }
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel_loc = viewport_upleft + (self.delta_u + self.delta_v) * 0.5;
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_loc
            + (self.delta_u * (i as f64 + offset.e[0]))
            + (self.delta_v * (j as f64 + offset.e[1]));
        Ray {
            ori: self.camera_center,
            dir: pixel_sample - self.camera_center,
        }
    }
    fn sample_square() -> Vec3 {
        Vec3 {
            e: [random_double_01() - 0.5, random_double_01() - 0.5, 0.0],
        }
    }
    pub fn render(&mut self, world: &HittableList, file: &mut File) {
        self.initialize();
        writeln!(file, "P3\n{} {}\n255", self.width, self.height).unwrap();
        for j in 0..self.height {
            for i in 0..self.width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth as i32, world);
                }
                write_color(&(pixel_color * self.pixel_samples_scale), file);
            }
        }
    }
}
