use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Vec3;

use crate::hittable_list::hittable::HitRecord;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}
impl Material {
    pub fn clone(&self) -> Self {
        match *self {
            Material::Lambertian { albedo } => Material::Lambertian { albedo: albedo },
            Material::Metal { albedo, fuzz } => Material::Metal { albedo, fuzz },
            Material::Dielectric { refraction_index } => Material::Dielectric { refraction_index },
        }
    }
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        match *self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                (
                    albedo,
                    Ray {
                        ori: rec.p,
                        dir: scatter_direction,
                    },
                    true,
                )
            }
            Material::Metal { albedo, fuzz } => {
                let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
                reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * fuzz;
                let scattered = Ray {
                    ori: rec.p,
                    dir: reflected,
                };
                let flag = Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
                (albedo, scattered, flag)
            }
            Material::Dielectric { refraction_index } => {
                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    refraction_index
                };
                let unit_direction = Vec3::unit_vector(r_in.dir);

                let cos_theta = Vec3::dot(&(-unit_direction), &rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;
                let direction = if cannot_refract || reflectance(cos_theta, ri) > random_double_01()
                {
                    Vec3::reflect(unit_direction, rec.normal)
                } else {
                    Vec3::refract(unit_direction, rec.normal, ri)
                };
                let scattered = Ray {
                    ori: rec.p,
                    dir: direction,
                };
                (Color { e: [1.0, 1.0, 1.0] }, scattered, true)
            }
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
/*
impl Dielectric {
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
pub struct Dielectric {
    pub refraction_index: f64,
}*/
