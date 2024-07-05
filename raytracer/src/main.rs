mod camera;
mod hittable_list;
mod rtweekend;
//mod sphere;

use std::fs::{self, File};
use std::path::Path;

use crate::camera::Camera;
use crate::hittable_list::material::Material;
use crate::hittable_list::HitObject;
use crate::hittable_list::HittableList;
//use crate::hittable_list::material::Dielectric;
//use crate::hittable_list::material::Lambertian;
//use crate::hittable_list::material::Metal;

use crate::rtweekend::random_double;
use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::sphere::Sphere;

fn main() {
    let path = Path::new("output/book1/image7.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();

    let mut world = HittableList::new();
    let material_ground = Material::Lambertian {
        albedo: Color { e: [0.5, 0.5, 0.5] },
    };
    world.add(HitObject::Sphere {
        center_st: Point3 {
            e: [0.0, -1000.0, -1.0],
        },
        radius: 1000.0,
        mat: material_ground,
        is_moving: false,
        center_vec: Vec3::new(),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_01();
            let center = Point3 {
                e: [
                    a as f64 + 0.9 * random_double_01(),
                    0.2,
                    b as f64 + 0.9 * random_double_01(),
                ],
            };
            if (center - Point3 { e: [4.0, 0.2, 0.0] }).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_01() * Vec3::random_01();

                    world.add(HitObject::Sphere {
                        center_st: center,
                        radius: 0.2,
                        mat: Material::Lambertian { albedo },
                        is_moving: true,
                        center_vec: Vec3 {
                            e: [0.0, random_double(0.0, 0.5), 0.0],
                        },
                    });
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rtweekend::random_double(0.0, 0.5);
                    world.add(HitObject::Sphere {
                        center_st: center,
                        radius: 0.2,
                        mat: Material::Metal { albedo, fuzz },
                        is_moving: false,
                        center_vec: Vec3::new(),
                    });
                } else {
                    world.add(HitObject::Sphere {
                        center_st: center,
                        radius: 0.2,
                        mat: Material::Dielectric {
                            refraction_index: 1.5,
                        },
                        is_moving: false,
                        center_vec: Vec3::new(),
                    });
                }
            }
        }
    }

    let material1 = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.add(HitObject::Sphere {
        center_st: Point3 { e: [0.0, 1.0, 0.0] },
        radius: 1.0,
        mat: material1,
        is_moving: false,
        center_vec: Vec3::new(),
    });
    let material2 = Material::Lambertian {
        albedo: Color { e: [0.4, 0.2, 0.1] },
    };
    world.add(HitObject::Sphere {
        center_st: Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        radius: 1.0,
        mat: material2,
        is_moving: false,
        center_vec: Vec3::new(),
    });
    let material3 = Material::Metal {
        albedo: Color { e: [0.7, 0.6, 0.5] },
        fuzz: 0.0,
    };
    world.add(HitObject::Sphere {
        center_st: Point3 { e: [4.0, 1.0, 0.0] },
        radius: 1.0,
        mat: material3,
        is_moving: false,
        center_vec: Vec3::new(),
    });

    let mut cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        width: 400,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        lookfrom: Point3 {
            e: [13.0, 2.0, 3.0],
        },
        lookat: Point3 { e: [0.0, 0.0, 0.0] },
        vup: Vec3 { e: [0.0, 1.0, 0.0] },

        defocus_angle: 0.6,
        focus_dist: 10.0,

        height: 0,
        camera_center: Vec3::new(),
        pixel_loc: Vec3::new(),
        delta_u: Vec3::new(),
        delta_v: Vec3::new(),
        pixel_samples_scale: 0.0,
        u: Vec3::new(),
        v: Vec3::new(),
        w: Vec3::new(),
        defocus_disk_u: Vec3::new(),
        defocus_disk_v: Vec3::new(),
    };
    cam.render(world, &mut file, 16);
}
