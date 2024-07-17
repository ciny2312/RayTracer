use crate::rtweekend::random_double_01;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;

use crate::onb::pdf::Pdf;
use crate::onb::Onb;

use crate::hittable_list::hittable::HitRecord;
use crate::hittable_list::texture::Texture;
//use crate::hittable_list::texture::Texture::SolidColor;

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian { tex: Box<Texture> },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
    Diffuselight { tex: Box<Texture> },
    Isotropic { tex: Box<Texture> },
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
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        match self {
            Material::Lambertian { tex } => {
                srec.attenuation = tex.value(rec.u, rec.v, &rec.p);
                srec.pdf_ptr = Box::new(Pdf::Cosinepdf {
                    uvw: Onb::build_from_w(rec.normal),
                });
                srec.skip_pdf = false;
                true
            }
            Material::Metal { albedo, fuzz } => {
                let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
                reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * (*fuzz);
                srec.attenuation = *albedo;
                srec.skip_pdf = true;
                srec.skip_pdf_ray = Ray {
                    ori: rec.p,
                    dir: reflected,
                    tm: r_in.tm,
                };
                true
            }
            Material::Dielectric { refraction_index } => {
                srec.attenuation = Color { e: [1.0, 1.0, 1.0] };
                srec.skip_pdf = true;

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
                srec.skip_pdf_ray = Ray {
                    ori: rec.p,
                    dir: direction,
                    tm: r_in.tm,
                };
                true
            }
            Material::Diffuselight { tex: _ } => false,
            Material::Isotropic { tex } => {
                srec.attenuation = tex.value(rec.u, rec.v, &rec.p);
                srec.pdf_ptr = Box::new(Pdf::Spherepdf);
                srec.skip_pdf = false;
                true
            }
        }
    }

    pub fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Material::Lambertian { tex: _ } => Color::new(),
            Material::Metal { albedo: _, fuzz: _ } => Color::new(),
            Material::Dielectric {
                refraction_index: _,
            } => Color::new(),
            Material::Diffuselight { tex } => {
                if !rec.front_face {
                    return Color::new();
                }
                tex.value(u, v, p)
            }
            Material::Isotropic { tex: _ } => Color::new(),
        }
    }
    pub fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian { tex: _ } => {
                let cosine = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.dir));
                if cosine < 0.0 {
                    0.0
                } else {
                    cosine / std::f64::consts::PI
                }
            }
            Material::Metal { albedo: _, fuzz: _ } => 0.0,
            Material::Dielectric {
                refraction_index: _,
            } => 0.0,
            Material::Diffuselight { tex: _ } => 0.0,
            Material::Isotropic { tex: _ } => 1.0 / (4.0 * std::f64::consts::PI),
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Box<Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}
