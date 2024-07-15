use indicatif::ProgressBar;
use std::fs::File;
use std::io::Write;

use std::sync::{Arc, Mutex};
use std::thread;
//use std::sync::mpsc::channel;
//use std::time::Instant;

use crate::hittable_list::HitObject;
//use crate::hittable_list::HittableList;
use crate::rtweekend::color::write_color;
use crate::rtweekend::degrees_to_radians;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double;
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
    pub background: Color,

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pub height: u32, // Rendered image height
    pub pixel_samples_scale: f64,
    pub sqrt_spp: u32,
    pub recip_sqrt_spp: f64,

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
    fn clone(&self) -> Camera {
        Camera {
            aspect_ratio: self.aspect_ratio, // Ratio of image width over height
            width: self.width,               // Rendered image width in pixel count
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            background: self.background,

            vfov: self.vfov,
            lookfrom: self.lookfrom,
            lookat: self.lookat,
            vup: self.vup,

            defocus_angle: self.defocus_angle,
            focus_dist: self.focus_dist,

            height: self.height,
            pixel_samples_scale: self.pixel_samples_scale,
            sqrt_spp: self.sqrt_spp,
            recip_sqrt_spp: self.recip_sqrt_spp,
            camera_center: self.camera_center,
            pixel_loc: self.pixel_loc,
            delta_u: self.delta_u,
            delta_v: self.delta_v,
            u: self.u,
            v: self.v,
            w: self.w,
            defocus_disk_u: self.defocus_disk_u,
            defocus_disk_v: self.defocus_disk_v,
        }
    }
    fn ray_color(&self, r: &Ray, depth: i32, world: &HitObject) -> Color {
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
            let color_from_emission = rec.mat.emitted(r, &rec, rec.u, rec.v, &rec.p);

            let mut pdf = 0.0; // = scattering_pdf;
            let (attenuation, _scattered, flag1) = rec.mat.scatter(r, &rec, &mut pdf);
            //    dbg!(pdf);
            if flag1 {
                let on_light = Point3 {
                    e: [
                        random_double(213.0, 343.0),
                        554.0,
                        random_double(227.0, 332.0),
                    ],
                };
                let mut to_light = on_light - rec.p;
                let distance_squared = to_light.sq_length();
                to_light = Vec3::unit_vector(to_light);
                if Vec3::dot(&to_light, &rec.normal) < 0.0 {
                    return color_from_emission;
                }
                let light_area = ((343 - 213) * (332 - 227)) as f64;
                let light_cosine = to_light.e[1].abs();
                if light_cosine < 0.000001 {
                    return color_from_emission;
                }
                pdf = distance_squared / (light_cosine * light_area);
                let scattered = Ray {
                    ori: rec.p,
                    dir: to_light,
                    tm: r.tm,
                };
                let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);
                let color_from_scatter =
                    (self.ray_color(&scattered, depth - 1, world) * attenuation * scattering_pdf)
                        / pdf;

                return color_from_emission + color_from_scatter;
            }
            return color_from_emission;
        }
        self.background
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

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as u32;
        self.pixel_samples_scale = 1.0 / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1.0 / self.sqrt_spp as f64;

        self.camera_center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.width as f64 / self.height as f64);

        self.w = Vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = Vec3::unit_vector(Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

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
    fn get_ray(&self, i: u32, j: u32, si: u32, sj: u32) -> Ray {
        let offset = self.sample_square_stratified(si, sj);
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
            tm: random_double_01(),
        }
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.e[0]) + (self.defocus_disk_v * p.e[1])
    }

    fn sample_square_stratified(&self, si: u32, sj: u32) -> Vec3 {
        let px = (si as f64 + random_double_01()) * self.recip_sqrt_spp - 0.5;
        let py = (sj as f64 + random_double_01()) * self.recip_sqrt_spp - 0.5;
        Vec3 { e: [px, py, 0.0] }
    }

    fn _sample_square() -> Vec3 {
        Vec3 {
            e: [random_double_01() - 0.5, random_double_01() - 0.5, 0.0],
        }
    }
    /*    pub fn render(&mut self, world: &HittableList, file: &mut File) {
        self.initialize();
        let total_pixels=self.height * self.width;
        let progress =ProgressBar::new(total_pixels as u64);
        writeln!(file, "P3\n{} {}\n255", self.width, self.height).unwrap();
        for j in 0..self.height {
            for i in 0..self.width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + Self::ray_color(&r, self.max_depth as i32, world);
                }
                write_color(&(pixel_color * self.pixel_samples_scale), file);
                progress.inc(1);
            }
        }
        progress.finish();
    }*/
    fn render_block(
        &self,
        world: &HitObject,
        start_y: u32,
        end_y: u32,
        result: Arc<Mutex<Vec<Color>>>,
    ) {
        for j in start_y..end_y {
            for i in 0..self.width {
                let mut pixel_color = Color::new();
                for sj in 0..self.sqrt_spp {
                    for si in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, si, sj);
                        pixel_color =
                            pixel_color + self.ray_color(&r, self.max_depth as i32, world);
                    }
                }
                let mut buffer = result.lock().unwrap();
                buffer[(j * self.width + i) as usize] = pixel_color * self.pixel_samples_scale;
            }
        }
    }
    pub fn render(&mut self, world: HitObject, file: &mut File, num_threads: u32) {
        self.initialize();
        let total_pixels = self.height * self.width;
        let progress = ProgressBar::new(total_pixels as u64);

        let block_height = self.height / num_threads;
        let result = Arc::new(Mutex::new(vec![Color::new(); total_pixels as usize]));
        let handles: Vec<_> = (0..num_threads)
            .map(|i| {
                let world = world.clone();
                let start_y = i * block_height;
                let end_y = if i == num_threads - 1 {
                    self.height
                } else {
                    (i + 1) * block_height
                };

                let result = Arc::clone(&result);

                let cam = self.clone();
                thread::spawn(move || {
                    cam.render_block(&world, start_y, end_y, result);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        writeln!(file, "P3\n{} {}\n255", self.width, self.height).unwrap();

        let buffer = result.lock().unwrap();
        for i in 0..total_pixels {
            write_color(&buffer[i as usize], file);
            progress.inc(1);
        }

        progress.finish();
    }
}
