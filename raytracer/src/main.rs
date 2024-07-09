mod aabb;
mod camera;
mod hittable_list;
mod rtw_image;
mod rtweekend;

use std::fs::{self, File};
use std::path::Path;

use crate::camera::Camera;
//use crate::hittable_list::HitObject;
use crate::hittable_list::hittable::build_box;
use crate::hittable_list::hittable::build_quad;
use crate::hittable_list::hittable::build_rotate;
use crate::hittable_list::hittable::build_sphere;
use crate::hittable_list::hittable::build_translate;
use crate::hittable_list::hittable::bvh_node;
use crate::hittable_list::hittable::new_hittable_list;
use crate::hittable_list::material::Material;
//use crate::hittable_list::HittableList;
//use crate::hittable_list::material::Dielectric;
//use crate::hittable_list::material::Lambertian;
//use crate::hittable_list::material::Metal;
use crate::hittable_list::perlin::Perlin;
use crate::hittable_list::texture::Texture;
use crate::hittable_list::texture::Texture::SolidColor;

use crate::rtweekend::random_double;
use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::sphere::Sphere;
fn bouncing_spheres(file: &mut File) {
    let mut world = new_hittable_list();
    let checker = Texture::Checkertexture {
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
        background: Color {
            e: [0.70, 0.80, 1.00],
        },

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
fn my_paint(file: &mut File) {
    let path = "raytracer/src/paint2.jpg";
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
        background: Color {
            e: [0.70, 0.80, 1.00],
        },

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
fn simple_light(file: &mut File) {
    let mut world = new_hittable_list();
    let pertext = Box::new(Texture::Noisetexture {
        noise: Box::new(Perlin::build_perlin()),
        scale: 4.0,
    });
    world.add(build_sphere(
        Point3 {
            e: [0.0, -1000.0, 0.0],
        },
        Vec3::new(),
        1000.0,
        Material::Lambertian {
            tex: pertext.clone(),
        },
        false,
    ));
    world.add(build_sphere(
        Point3 { e: [0.0, 2.0, 0.0] },
        Vec3::new(),
        2.0,
        Material::Lambertian { tex: pertext },
        false,
    ));

    let difflight = Box::new(Texture::SolidColor {
        albedo: Color { e: [4.0, 4.0, 4.0] },
    });
    world.add(build_quad(
        Point3 {
            e: [3.0, 1.0, -2.0],
        },
        Vec3 { e: [2.0, 0.0, 0.0] },
        Vec3 { e: [0.0, 2.0, 0.0] },
        Material::Diffuselight {
            tex: difflight.clone(),
        },
    ));
    world.add(build_sphere(
        Point3 { e: [0.0, 7.0, 0.0] },
        Vec3::new(),
        2.0,
        Material::Diffuselight { tex: difflight },
        false,
    ));
    let mut cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(),

        vfov: 20.0,
        lookfrom: Point3 {
            e: [26.0, 3.0, 6.0],
        },
        lookat: Point3 { e: [0.0, 2.0, 0.0] },
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
fn cornell_box(file: &mut File) {
    let mut world = new_hittable_list();

    let red = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color {
                e: [0.65, 0.05, 0.05],
            },
        }),
    };
    let white = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color {
                e: [0.73, 0.73, 0.73],
            },
        }),
    };
    let green = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color {
                e: [0.12, 0.45, 0.15],
            },
        }),
    };
    let light = Material::Diffuselight {
        tex: Box::new(SolidColor {
            albedo: Color {
                e: [15.0, 15.0, 15.0],
            },
        }),
    };

    world.add(build_quad(
        Point3 {
            e: [555.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 555.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 555.0],
        },
        green,
    ));
    world.add(build_quad(
        Point3 { e: [0.0, 0.0, 0.0] },
        Vec3 {
            e: [0.0, 555.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 555.0],
        },
        red,
    ));
    world.add(build_quad(
        Point3 {
            e: [343.0, 554.0, 332.0],
        },
        Vec3 {
            e: [-130.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, -105.0],
        },
        light,
    ));
    world.add(build_quad(
        Point3 { e: [0.0, 0.0, 0.0] },
        Vec3 {
            e: [555.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 555.0],
        },
        white.clone(),
    ));
    world.add(build_quad(
        Point3 {
            e: [555.0, 555.0, 555.0],
        },
        Vec3 {
            e: [-555.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, -555.0],
        },
        white.clone(),
    ));
    world.add(build_quad(
        Point3 {
            e: [0.0, 0.0, 555.0],
        },
        Vec3 {
            e: [555.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 555.0, 0.0],
        },
        white.clone(),
    ));
    let box1 = build_box(
        &Point3 { e: [0.0, 0.0, 0.0] },
        &Vec3 {
            e: [165.0, 330.0, 165.0],
        },
        &white,
    );
    let box1 = build_rotate(&box1, 15.0);
    world.add(build_translate(
        &box1,
        Vec3 {
            e: [265.0, 0.0, 295.0],
        },
    ));

    let box2 = build_box(
        &Point3 { e: [0.0, 0.0, 0.0] },
        &Vec3 {
            e: [165.0, 165.0, 165.0],
        },
        &white,
    );
    let box2 = build_rotate(&box2, -18.0);
    world.add(build_translate(
        &box2,
        Vec3 {
            e: [130.0, 0.0, 65.0],
        },
    ));

    let mut cam = Camera {
        aspect_ratio: 1.0,
        width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::new(),

        vfov: 40.0,
        lookfrom: Point3 {
            e: [278.0, 278.0, -800.0],
        },
        lookat: Point3 {
            e: [278.0, 278.0, 0.0],
        },
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
    let mut objects = world.get_objects();
    let size = objects.len();
    let bvh_root = bvh_node(&mut objects, 0, size);
    cam.render(bvh_root, file, 16);
}

fn main() {
    let path = Path::new("output/book1/image6.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    bouncing_spheres(&mut file);

    let path = Path::new("output/book1/image7.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    my_paint(&mut file);

    let path = Path::new("output/book1/image9.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    simple_light(&mut file);
    let path = Path::new("output/book1/image10.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();
    cornell_box(&mut file);
}
