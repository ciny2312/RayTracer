mod aabb;
mod camera;
mod hittable_list;
mod rtw_image;
mod rtweekend;

use std::fs::{self, File};
use std::path::Path;

use crate::camera::Camera;
//use crate::hittable_list::HitObject;
use crate::hittable_list::hittable::_build_constant_medium;
use crate::hittable_list::hittable::_build_sphere;
use crate::hittable_list::hittable::build_box;
use crate::hittable_list::hittable::build_quad;
use crate::hittable_list::hittable::build_rotate;
use crate::hittable_list::hittable::build_translate;
use crate::hittable_list::hittable::bvh_node;
use crate::hittable_list::hittable::new_hittable_list;
use crate::hittable_list::material::Material;
//use crate::hittable_list::HittableList;
//use crate::hittable_list::material::_Dielectric;
//use crate::hittable_list::material::Lambertian;
//use crate::hittable_list::material::_Metal;
use crate::hittable_list::perlin::Perlin;
use crate::hittable_list::texture::Texture;
use crate::hittable_list::texture::Texture::SolidColor;

use crate::rtweekend::random_double;
//use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::sphere::Sphere;

fn _final_scene(file: &mut File) {
    let mut boxes1 = new_hittable_list();
    let ground = Material::Lambertian {
        tex: Box::new(Texture::SolidColor {
            albedo: Color {
                e: [0.48, 0.83, 0.53],
            },
        }),
    };
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(build_box(
                &Point3 { e: [x0, y0, z0] },
                &Point3 { e: [x1, y1, z1] },
                &ground,
            ));
        }
    }

    let mut world = new_hittable_list();

    let mut objects = boxes1.get_objects();
    let size = objects.len();
    world.add(bvh_node(&mut objects, 0, size));
    let light = Material::Diffuselight {
        tex: Box::new(SolidColor {
            albedo: Color { e: [7.0, 7.0, 7.0] },
        }),
    };
    world.add(build_quad(
        Point3 {
            e: [123.0, 554.0, 147.0],
        },
        Vec3 {
            e: [300.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 265.0],
        },
        light,
    ));
    let center1 = Point3 {
        e: [400.0, 400.0, 200.0],
    };
    let sphere_material = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color { e: [0.7, 0.3, 0.1] },
        }),
    };
    world.add(_build_sphere(
        center1,
        Vec3 {
            e: [30.0, 0.0, 0.0],
        },
        50.0,
        sphere_material,
        true,
    ));
    world.add(_build_sphere(
        Point3 {
            e: [260.0, 150.0, 45.0],
        },
        Vec3::new(),
        50.0,
        Material::_Dielectric {
            refraction_index: 1.5,
        },
        false,
    ));
    world.add(_build_sphere(
        Point3 {
            e: [0.0, 150.0, 145.0],
        },
        Vec3::new(),
        50.0,
        Material::_Metal {
            albedo: Color { e: [0.8, 0.8, 0.9] },
            fuzz: 1.0,
        },
        false,
    ));

    let boundary = _build_sphere(
        Point3 {
            e: [360.0, 150.0, 145.0],
        },
        Vec3::new(),
        70.0,
        Material::_Dielectric {
            refraction_index: 1.5,
        },
        false,
    );
    world.add(_build_constant_medium(
        &boundary,
        0.2,
        &Texture::SolidColor {
            albedo: Color { e: [0.2, 0.4, 0.9] },
        },
    ));
    world.add(boundary);

    let boundary = _build_sphere(
        Point3 { e: [0.0, 0.0, 0.0] },
        Vec3::new(),
        5000.0,
        Material::_Dielectric {
            refraction_index: 1.5,
        },
        false,
    );
    world.add(_build_constant_medium(
        &boundary,
        0.0001,
        &Texture::SolidColor {
            albedo: Color { e: [1.0, 1.0, 1.0] },
        },
    ));

    let path = "raytracer/src/paint2.jpg";
    let texture = rtw_image::_load_image_to_float_array(path);
    let surface = Material::Lambertian {
        tex: Box::new(texture),
    };
    world.add(_build_sphere(
        Point3 {
            e: [400.0, 200.0, 400.0],
        },
        Vec3::new(),
        100.0,
        surface,
        false,
    ));
    let pertext = Texture::_Noisetexture {
        noise: Box::new(Perlin::_build_perlin()),
        scale: 0.2,
    };
    world.add(_build_sphere(
        Point3 {
            e: [220.0, 280.0, 300.0],
        },
        Vec3::new(),
        80.0,
        Material::Lambertian {
            tex: Box::new(pertext),
        },
        false,
    ));

    let mut boxes2 = new_hittable_list();
    let white = Material::Lambertian {
        tex: Box::new(SolidColor {
            albedo: Color {
                e: [0.73, 0.73, 0.73],
            },
        }),
    };
    for _j in 0..1000 {
        boxes2.add(_build_sphere(
            Point3::random(0.0, 165.0),
            Vec3::new(),
            10.0,
            white.clone(),
            false,
        ));
    }
    let mut objects = boxes2.get_objects();
    let size = objects.len();
    world.add(build_translate(
        &build_rotate(&bvh_node(&mut objects, 0, size), 15.0),
        Vec3 {
            e: [-100.0, 270.0, 395.0],
        },
    ));

    let mut cam = Camera {
        aspect_ratio: 1.0,
        width: 800,
        samples_per_pixel: 5000,
        max_depth: 40,
        background: Color::new(),

        vfov: 40.0,
        lookfrom: Point3 {
            e: [478.0, 278.0, -600.0],
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
        sqrt_spp: 0,
        recip_sqrt_spp: 0.0,
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
    let path = Path::new("output/book1/image10.ppm");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut file = File::create(path).unwrap();

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
        samples_per_pixel: 64,
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
        sqrt_spp: 0,
        recip_sqrt_spp: 0.0,
        u: Vec3::new(),
        v: Vec3::new(),
        w: Vec3::new(),
        defocus_disk_u: Vec3::new(),
        defocus_disk_v: Vec3::new(),
    };
    let mut objects = world.get_objects();
    let size = objects.len();
    let bvh_root = bvh_node(&mut objects, 0, size);
    cam.render(bvh_root, &mut file, 16);
}
