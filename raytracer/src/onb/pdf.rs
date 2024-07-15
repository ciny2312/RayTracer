use crate::hittable_list::HitObject;
use crate::onb::Onb;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
pub enum Pdf {
    _Spherepdf,
    _Cosinepdf {
        uvw: Onb,
    },
    Hittablepdf {
        objects: Box<HitObject>,
        ori: Point3,
    },
}
impl Pdf {
    pub fn value(&self, dir: Vec3) -> f64 {
        match self {
            Pdf::_Spherepdf => 1.0 / (4.0 * std::f64::consts::PI),
            Pdf::_Cosinepdf { uvw } => {
                let cosine_theta = Vec3::dot(&Vec3::unit_vector(dir), &uvw.axis[2]);
                (cosine_theta / std::f64::consts::PI).max(0.0)
            }
            Pdf::Hittablepdf { objects, ori } => objects.pdf_value(*ori, dir),
        }
    }
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::_Spherepdf => Vec3::random_unit_vector(),
            Pdf::_Cosinepdf { uvw } => uvw.local(&Vec3::random_cosine_direction()),
            Pdf::Hittablepdf { objects, ori } => objects.random_from(*ori),
        }
    }
}
