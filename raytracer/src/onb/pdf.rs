use crate::hittable_list::HitObject;
use crate::onb::Onb;
use crate::rtweekend::random_double_01;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
pub enum Pdf {
    Spherepdf,
    Cosinepdf {
        uvw: Onb,
    },
    Hittablepdf {
        objects: Box<HitObject>,
        ori: Point3,
    },
    Mixturepdf {
        p: [Box<Pdf>; 2],
    },
}

impl Pdf {
    pub fn value(&self, dir: Vec3) -> f64 {
        match self {
            Pdf::Spherepdf => 1.0 / (4.0 * std::f64::consts::PI),
            Pdf::Cosinepdf { uvw } => {
                let cosine_theta = Vec3::dot(&Vec3::unit_vector(dir), &uvw.axis[2]);
                (cosine_theta / std::f64::consts::PI).max(0.0)
            }
            Pdf::Hittablepdf { objects, ori } => objects.pdf_value(*ori, dir),
            Pdf::Mixturepdf { p } => p[0].value(dir) * 0.5 + p[1].value(dir) * 0.5,
        }
    }
    pub fn generate(&self) -> Vec3 {
        match self {
            Pdf::Spherepdf => Vec3::random_unit_vector(),
            Pdf::Cosinepdf { uvw } => uvw.local(&Vec3::random_cosine_direction()),
            Pdf::Hittablepdf { objects, ori } => objects.random_from(*ori),
            Pdf::Mixturepdf { p } => {
                if random_double_01() < 0.5 {
                    p[0].generate()
                } else {
                    p[1].generate()
                }
            }
        }
    }
}
