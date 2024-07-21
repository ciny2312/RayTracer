mod aabb;
mod camera;
mod hittable_list;
mod onb;
mod rtw_image;
mod rtweekend;
mod obj;

use std::fs::{self, File};
use std::path::Path;

use hittable_list::hittable::build_constant_medium;
use hittable_list::texture::Texture;

use crate::camera::Camera;
//use crate::hittable_list::HitObject;
//use crate::hittable_list::hittable::_build_constant_medium;
use crate::hittable_list::hittable::build_box;
use crate::hittable_list::hittable::build_quad;
use crate::hittable_list::hittable::build_rotate;
use crate::hittable_list::hittable::build_sphere;
use crate::hittable_list::hittable::build_translate;
use crate::hittable_list::hittable::build_triangle;
use crate::hittable_list::hittable::bvh_node;
use crate::hittable_list::hittable::new_hittable_list;
use crate::hittable_list::material::Material;
//use crate::hittable_list::HittableList;
//use crate::hittable_list::material::_Dielectric;
//use crate::hittable_list::material::Lambertian;
//use crate::hittable_list::material::_Metal;
//use crate::hittable_list::perlin::Perlin;
//use crate::hittable_list::texture::Texture;
use crate::hittable_list::texture::Texture::SolidColor;

//use crate::rtweekend::random_double;
//use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::sphere::Sphere;

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

/*   world.add(build_quad(
        Point3 {
            e: [555.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 555.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 555.0],
        },
        white.clone(),
    ));
    world.add(build_quad(
        Point3 { e: [0.0, 0.0, 0.0] },
        Vec3 {
            e: [0.0, 555.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 555.0],
        },
        white.clone(),
    ));
*/
    let mut lights = new_hittable_list();
/*    lights.add(build_quad(
        Point3 {
            e: [343.0, 554.0, 332.0],
        },
        Vec3 {
            e: [-130.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, -105.0],
        },
        light.clone(),
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
        light.clone(),
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
    ));*/
    world.add(build_quad(
        Point3 {
            e: [-1000.0, -180.0, -300.0],
        },
        Vec3 {
            e: [1500.0, 0.0, 0.0],
        },
        Vec3 {
            e: [0.0, 0.0, 2000.0],
        },
        white.clone(),
    ));
    let aluminum = Material::Metal {
        albedo: Color {
            e: [0.8, 0.85, 0.88],
        },
        fuzz: 0.0,
    };
    world.add(build_sphere(
        Point3 {
            e: [90.0, 20.0, -200.0],
        },
        Vec3::new(),
        10.0,
        aluminum.clone(),
        false,
    ));
    world.add(build_sphere(
        Point3 {
            e: [20.0, -170.0, 100.0],
        },
        Vec3::new(),
        10.0,
        aluminum.clone(),
        false,
    ));
    
    let aluminum_r = Material::Metal {
        albedo: Color {
            e: [0.8, 0.55, 0.88],
        },
        fuzz: 0.0,
    };
    world.add(build_sphere(
        Point3 {
            e: [90.0, 70.0, -80.0],
        },
        Vec3::new(),
        15.0,
        aluminum_r.clone(),
        false,
    ));
    world.add(build_sphere(
        Point3 {
            e: [50.0, 50.0, 200.0],
        },
        Vec3::new(),
        10.0,
        aluminum_r.clone(),
        false,
    ));

    let glass = Material::Dielectric {
        refraction_index: 1.5,
    };
    world.add(build_sphere(
        Point3 {
            e: [90.0, -20.0, 100.0],
        },
        Vec3::new(),
        10.0,
        glass.clone(),
        false,
    ));
    world.add(build_sphere(
        Point3 {
            e: [-100.0, -150.0, -100.0],
        },
        Vec3::new(),
        30.0,
        glass.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [-90.0, -170.0, 120.0],
        },
        Vec3::new(),
        20.0,
        glass.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [0.0, 280.0, -250.0],
        },
        Vec3::new(),
        100.0,
        light.clone(),
        false,
    ));
    lights.add(build_sphere(
        Point3 {
            e: [0.0, 280.0, -250.0],
        },
        Vec3::new(),
        100.0,
        light.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [90.0, -90.0, -50.0],
        },
        Vec3::new(),
        5.0,
        light.clone(),
        false,
    ));
    lights.add(build_sphere(
        Point3 {
            e: [90.0, -90.0, -50.0],
        },
        Vec3::new(),
        5.0,
        light.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [100.0, 80.0, 0.0],
        },
        Vec3::new(),
        15.0,
        light.clone(),
        false,
    ));
    lights.add(build_sphere(
        Point3 {
            e: [100.0, 100.0, 0.0],
        },
        Vec3::new(),
        15.0,
        light.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [-50.0, 20.0, 100.0],
        },
        Vec3::new(),
        15.0,
        light.clone(),
        false,
    ));
    lights.add(build_sphere(
        Point3 {
            e: [-50.0, 20.0, 100.0],
        },
        Vec3::new(),
        15.0,
        light.clone(),
        false,
    ));

    world.add(build_sphere(
        Point3 {
            e: [-30.0, 0.0, -100.0],
        },
        Vec3::new(),
        20.0,
        light.clone(),
        false,
    ));
    lights.add(build_sphere(
        Point3 {
            e: [-30.0, 0.0, -100.0],
        },
        Vec3::new(),
        20.0,
        light.clone(),
        false,
    ));

    world.add(obj::read_from_obj());

    let mut cam = Camera {
        aspect_ratio: 1.0,
        width: 800,
        samples_per_pixel: 200,
        max_depth: 20,
        background: Color{e:[0.3,0.2,0.2]},

        vfov: 40.0,
        lookfrom: Point3 {
            e: [-800.0, 0.0, 0.0],
        },
        lookat: Point3 {
            e: [0.0, 0.0, 0.0],
        },
        vup: Vec3 { e: [0.0, 1.0, 0.0] },

        defocus_angle: 0.0,
        focus_dist: 800.0,

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
    cam.render(bvh_root, lights, &mut file, 8);
}
