pub mod pdf;
use crate::rtweekend::vec3::Vec3;
pub struct Onb {
    pub axis: [Vec3; 3],
}
impl Onb {
    pub fn local(&self, x: &Vec3) -> Vec3 {
        self.axis[0] * x.e[0] + self.axis[1] * x.e[1] + self.axis[2] * x.e[2]
    }
    pub fn build_from_w(w: Vec3) -> Onb {
        let unit_w = Vec3::unit_vector(w);
        let a = if unit_w.e[0].abs() > 0.9 {
            Vec3 { e: [0.0, 1.0, 0.0] }
        } else {
            Vec3 { e: [1.0, 0.0, 0.0] }
        };
        let v = Vec3::unit_vector(Vec3::cross(&unit_w, &a));
        let u = Vec3::cross(&unit_w, &v);
        Onb {
            axis: [u, v, unit_w],
        }
    }
}
