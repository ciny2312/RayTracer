use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;

use crate::onb::Onb;

use crate::hittable_list::hittable::HitRecord;
use crate::hittable_list::texture::Texture;
//use crate::hittable_list::texture::Texture::SolidColor;

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian { tex: Box<Texture> },
    _Metal { albedo: Color, fuzz: f64 },
    _Dielectric { refraction_index: f64 },
    Diffuselight { tex: Box<Texture> },
    _Isotropic { tex: Box<Texture> },
}
impl Material {
    /*    pub fn clone(&self) -> Material {
        match self {
            Material::Lambertian { tex } => Material::Lambertian { tex: tex.clone() },
            Material::_Metal { albedo, fuzz } => Material::_Metal {
                albedo: *albedo,
                fuzz: *fuzz,
            },
            Material::_Dielectric { refraction_index } => Material::_Dielectric {
                refraction_index: *refraction_index,
            },
        }
    }*/
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, pdf: &mut f64) -> (Color, Ray, bool) {
        match self {
            Material::Lambertian { tex } => {
                let uvw = Onb::build_from_w(rec.normal);
                let scatter_direction = uvw.local(&Vec3::random_cosine_direction());

                let scattered = Ray {
                    ori: rec.p,
                    dir: Vec3::unit_vector(scatter_direction),
                    tm: r_in.tm,
                };
                *pdf = Vec3::dot(&uvw.axis[2], &scattered.dir) / std::f64::consts::PI;
                (tex.value(rec.u, rec.v, &rec.p), scattered, true)
            }
            Material::_Metal { albedo, fuzz } => {
                let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
                reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * (*fuzz);
                let scattered = Ray {
                    ori: rec.p,
                    dir: reflected,
                    tm: r_in.tm,
                };
                let flag = Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
                (*albedo, scattered, flag)
            }
            Material::_Dielectric { refraction_index } => {
                let ri = if rec.front_face {
                    1.0 / (*refraction_index)
                } else {
                    *refraction_index
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
                    tm: r_in.tm,
                };
                (Color { e: [1.0, 1.0, 1.0] }, scattered, true)
            }
            Material::Diffuselight { tex: _ } => (Color::new(), Ray::new(), false),
            Material::_Isotropic { tex } => {
                *pdf = 1.0 / (4.0 * std::f64::consts::PI);
                (
                    tex.value(rec.u, rec.v, &rec.p),
                    Ray {
                        ori: rec.p,
                        dir: Vec3::random_unit_vector(),
                        tm: r_in.tm,
                    },
                    true,
                )
            }
        }
    }

    pub fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Material::Lambertian { tex: _ } => Color::new(),
            Material::_Metal { albedo: _, fuzz: _ } => Color::new(),
            Material::_Dielectric {
                refraction_index: _,
            } => Color::new(),
            Material::Diffuselight { tex } => {
                if !rec.front_face {
                    return Color::new();
                }
                tex.value(u, v, p)
            }
            Material::_Isotropic { tex: _ } => Color::new(),
        }
    }
    pub fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian { tex: _ } => 1.0 / (2.0 * std::f64::consts::PI),
            Material::_Metal { albedo: _, fuzz: _ } => 0.0,
            Material::_Dielectric {
                refraction_index: _,
            } => 0.0,
            Material::Diffuselight { tex: _ } => 0.0,
            Material::_Isotropic { tex: _ } => 1.0 / (4.0 * std::f64::consts::PI),
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

/*
impl _Dielectric {
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}
pub struct _Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
pub struct _Dielectric {
    pub refraction_index: f64,
}*/
