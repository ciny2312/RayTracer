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
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray, bool) {
        let reflected = Vec3::reflect(r_in.dir, rec.normal);
        (
            self.albedo,
            Ray {
                ori: rec.p,
                dir: reflected,
            },
            true,
        )
    }
}
