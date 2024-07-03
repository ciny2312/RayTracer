use std::fs::File;
use std::io::Write;

use crate::hittable_list::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::rtweekend::color::write_color;
use crate::rtweekend::degrees_to_radians;
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

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pub height: u32, // Rendered image height
    pub pixel_samples_scale: f64,
    pub camera_center: Point3, // Camera center
    pub pixel_loc: Point3,     // Location of pixel 0, 0
    pub delta_u: Vec3,         // Offset to pixel to the right
    pub delta_v: Vec3,         // Offset to pixel below
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
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
            let (attenuation, scattered, flag1) = rec.mat.scatter(r, &rec);
            if flag1 {
                return Self::ray_color(&scattered, depth - 1, world) * attenuation;
            }
            return Color::new();
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

        self.camera_center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(self.vup, self.w));
        self.v = Vec3::cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        self.delta_u = viewport_u / (self.width as f64);
        self.delta_v = viewport_v / (self.height as f64);

        let viewport_upleft =
            self.camera_center - self.w * self.focus_dist - viewport_u * 0.5 - viewport_v * 0.5;

        self.pixel_loc = viewport_upleft + (self.delta_u + self.delta_v) * 0.5;

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel_loc
            + (self.delta_u * (i as f64 + offset.e[0]))
            + (self.delta_v * (j as f64 + offset.e[1]));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        Ray {
            ori: ray_origin,
            dir: pixel_sample - ray_origin,
        }
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.e[0]) + (self.defocus_disk_v * p.e[1])
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
