use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;
//use crate::rtweekend::vec3::Color;
pub struct Ray {
    pub dir: Vec3,
    pub ori: Point3,
    pub tm: f64,
}
impl Ray {
    pub fn new() -> Ray {
        Ray {
            dir: Vec3::new(),
            ori: Vec3::new(),
            tm: 0.0,
        }
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.ori + self.dir * t
    }
    pub fn _clone(&self) -> Self {
        Self {
            dir: self.dir,
            ori: self.ori,
            tm: self.tm,
        }
    }
}
