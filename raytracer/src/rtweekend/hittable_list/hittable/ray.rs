pub mod vec3;
pub struct Ray {
    dir: Vec3,
    ori: Point3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        Vec3::add(&self.ori, &Vec3::mul(&self.dir, t))
    }
    pub fn _clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            ori: self.ori.clone(),
        }
    }
}