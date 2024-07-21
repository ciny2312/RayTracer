use crate::{hittable_list::{hittable::new_hittable_list, HitObject}, rtweekend::{random_double, vec3::Point3}};
use crate::rtweekend::vec3::Color;
use crate::hittable_list::texture::Texture;
use crate::hittable_list::material::Material;
use crate::hittable_list::hittable::build_triangle;
pub fn read_from_obj()->HitObject{
    let mut world=new_hittable_list();

    let obj_file = "raytracer/sources/rose.obj";
    let (models, materials) =
        tobj::load_obj(
            &obj_file,
            &tobj::LoadOptions::default()
        )
        .expect("Failed to OBJ load file");
    let materials = materials.expect("Failed to load MTL file");

    let mut p=Vec::new();
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        assert!(mesh.positions.len() % 3 == 0);

        for vtx in 0..mesh.positions.len() / 3 {
            let v1=Point3{
                e:[mesh.positions[3 * vtx] as f64+0.0,
                mesh.positions[3 * vtx + 1] as f64-20.0,
                mesh.positions[3 * vtx + 2] as f64+0.0]};
            p.push(v1*3.0);
        }
        let mut next_face = 0;
        for face in 0..(mesh.face_arities.len()-1) {
            let end = next_face + mesh.face_arities[face] as usize;
    
            let face_indices = &mesh.indices[next_face..end];
            
            let g=random_double(0.0,0.4);
            let b=random_double(0.0,0.4);
            let red = Material::Lambertian {
                tex: Box::new(Texture::SolidColor {
                    albedo: Color {
                        e: [0.9, g, b],
                    },
                }),
            };
            let aluminum = Material::Metal {
                albedo: Color {
                    e: [0.9, g, b],
                },
                fuzz: 0.0,
            };
            world.add(build_triangle(p[face_indices[0] as usize],
                p[face_indices[1] as usize],
                p[face_indices[2] as usize],aluminum));
    
            next_face = end;
        }
    }
/*    for i in 0..(p.len()-2){
        let g=random_double(0.0,0.2);
        let b=random_double(0.0,0.2);
        let red = Material::Lambertian {
            tex: Box::new(Texture::SolidColor {
                albedo: Color {
                    e: [0.9, g, b],
                },
            }),
        };
        world.add(build_triangle(p[i],p[i+1],p[i+2],red));
    }*/
    world
}