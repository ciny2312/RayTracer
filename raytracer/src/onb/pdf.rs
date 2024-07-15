use crate::onb::Onb;
use crate::rtweekend::vec3::Vec3;
pub enum Pdf {
    _SpherePdf,
    CosinePdf { uvw: Onb },
}
impl Pdf {
    pub fn value(&self, dir: Vec3) -> f64 {
        match self {
            Pdf::_SpherePdf => 1.0 / (4.0 * std::f64::consts::PI),
            Pdf::CosinePdf { uvw } => {
                let cosine_theta = Vec3::dot(&Vec3::unit_vector(dir), &uvw.axis[2]);
                (cosine_theta / std::f64::consts::PI).max(0.0)
            }
        }
    }
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::_SpherePdf => Vec3::random_unit_vector(),
            Pdf::CosinePdf { uvw } => uvw.local(&Vec3::random_cosine_direction()),
        }
    }
}
