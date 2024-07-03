use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Vec3;

use crate::hittable_list::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool);
}

pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        (
            self.albedo,
            Ray {
                ori: rec.p,
                dir: scatter_direction,
            },
            true,
        )
    }
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let mut reflected = Vec3::reflect(r_in.dir, rec.normal);
        reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray {
            ori: rec.p,
            dir: reflected,
        };
        let flag = Vec3::dot(&scattered.dir, &rec.normal) > 0.0;
        (self.albedo, scattered, flag)
    }
}
pub struct Dielectric {
    pub refraction_index: f64,
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = Vec3::unit_vector(r_in.dir);
        let refracted = Vec3::refract(unit_direction, rec.normal, ri);
        let scattered = Ray {
            ori: rec.p,
            dir: refracted,
        };
        (Color { e: [1.0, 1.0, 1.0] }, scattered, true)
    }
}
