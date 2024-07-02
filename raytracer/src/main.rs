mod camera;
mod hittable_list;
mod rtweekend;
mod sphere;

use std::fs::{self, File};
use std::path::Path;
use std::sync::Arc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::rtweekend::vec3::Point3;
use crate::sphere::Sphere;
//use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Vec3;

fn main() {
    let path = Path::new("output/book1/image6.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();

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

    let mut cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        width: 400,
        samples_per_pixel: 100,
        max_depth:50,
        height: 0,
        camera_center: Vec3::new(),
        pixel_loc: Vec3::new(),
        delta_u: Vec3::new(),
        delta_v: Vec3::new(),
        pixel_samples_scale: 0.0,
    };
    cam.render(&world, &mut file);
}
