mod aabb;
mod camera;
mod hittable_list;
mod rtw_image;
mod rtweekend;

use std::fs::{self, File};
use std::path::Path;

use crate::camera::Camera;
//use crate::hittable_list::HitObject;
use crate::hittable_list::hittable::build_sphere;
use crate::hittable_list::hittable::bvh_node;
use crate::hittable_list::hittable::new_hittable_list;
use crate::hittable_list::material::Material;
//use crate::hittable_list::HittableList;
//use crate::hittable_list::material::Dielectric;
//use crate::hittable_list::material::Lambertian;
//use crate::hittable_list::material::Metal;
use crate::hittable_list::texture::Texture::Checkertexture;
use crate::hittable_list::texture::Texture::SolidColor;

use crate::rtweekend::random_double;
use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::sphere::Sphere;
fn bouncing_spheres(file: &mut File) {
    let mut world = new_hittable_list();
    let checker = Checkertexture {
        inv_scale: 1.0 / 0.32,
        even: Box::new(SolidColor {
            albedo: Color { e: [0.2, 0.3, 0.1] },
        }),
        odd: Box::new(SolidColor {
            albedo: Color { e: [0.9, 0.9, 0.9] },
        }),
    };
    let material_ground = Material::Lambertian {
        tex: Box::new(checker),
    };
    world.add(build_sphere(
        Point3 {
            e: [0.0, -1000.0, -1.0],
        },
        Vec3::new(),
        1000.0,
        material_ground,
        false,
    ));

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

                    world.add(build_sphere(
                        center,
                        Vec3 {
                            e: [0.0, random_double(0.0, 0.5), 0.0],
                        },
                        0.2,
                        Material::Lambertian {
                            tex: Box::new(SolidColor { albedo }),
                        },
                        true,
                    ));
                //    dbg!("ball1");
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rtweekend::random_double(0.0, 0.5);
                    world.add(build_sphere(
                        center,
                        Vec3::new(),
                        0.2,
                        Material::Metal { albedo, fuzz },
                        false,
                    ));
                //    dbg!("ball2");
                } else {
                    world.add(build_sphere(
                        center,
                        Vec3::new(),
                        0.2,
                        Material::Dielectric {
                            refraction_index: 1.5,
                        },
                        false,
                    ));
                    //    dbg!("ball3");
                }
            }
        }
    }
    let material1 = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.add(build_sphere(
        Point3 { e: [0.0, 1.0, 0.0] },
        Vec3::new(),
        1.0,
        material1,
        false,
    ));
    let material2 = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color { e: [0.4, 0.2, 0.1] },
        }),
    };
    world.add(build_sphere(
        Point3 {
            e: [-4.0, 1.0, 0.0],
        },
        Vec3::new(),
        1.0,
        material2,
        false,
    ));
    let material3 = Material::Metal {
        albedo: Color { e: [0.7, 0.6, 0.5] },
        fuzz: 0.0,
    };
    world.add(build_sphere(
        Point3 { e: [4.0, 1.0, 0.0] },
        Vec3::new(),
        1.0,
        material3,
        false,
    ));

    let mut objects = world.get_objects();
    let size = objects.len();
    let bvh_root = bvh_node(&mut objects, 0, size);
    //    dbg!(bvh_root.clone());
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
    cam.render(bvh_root, file, 16);
}
fn _checkered_spheres(file: &mut File) {
    let mut world = new_hittable_list();
    let checker = Checkertexture {
        inv_scale: 1.0 / 0.32,
        even: Box::new(SolidColor {
            albedo: Color { e: [0.2, 0.3, 0.1] },
        }),
        odd: Box::new(SolidColor {
            albedo: Color { e: [0.9, 0.9, 0.9] },
        }),
    };
    let material_ground = Material::Lambertian {
        tex: Box::new(checker),
    };
    world.add(build_sphere(
        Point3 {
            e: [0.0, -10.0, 0.0],
        },
        Vec3::new(),
        10.0,
        material_ground.clone(),
        false,
    ));
    world.add(build_sphere(
        Point3 {
            e: [0.0, 10.0, 0.0],
        },
        Vec3::new(),
        10.0,
        material_ground,
        false,
    ));
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

        defocus_angle: 0.0,
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
    cam.render(world, file, 16);
}
fn my_paint(file: &mut File) {
    let path = "raytracer/src/paint1.jpg";
    let texture = rtw_image::load_image_to_float_array(path);
    let surface = Material::Lambertian {
        tex: Box::new(texture),
    };
    let globe = build_sphere(Point3::new(), Vec3::new(), 2.0, surface, false);

    let mut cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        width: 400,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        lookfrom: Point3 {
            e: [0.0, 0.0, 12.0],
        },
        lookat: Point3 { e: [0.0, 0.0, 0.0] },
        vup: Vec3 { e: [0.0, 1.0, 0.0] },

        defocus_angle: 0.0,
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
    cam.render(globe, file, 16);
}
fn main() {
    let path = Path::new("output/book1/image7.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    bouncing_spheres(&mut file);
    let path = Path::new("output/book1/image8.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    my_paint(&mut file);
}
