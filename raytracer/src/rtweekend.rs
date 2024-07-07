use rand::Rng;

pub mod color;
pub mod interval;
pub mod ray;
pub mod vec3;
pub const INF: f64 = 1e18;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}
pub fn random_double_01() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
pub fn random_double(min: f64, max: f64) -> f64 {
    min + random_double_01() * (max - min)
}
pub fn random_int(min: i32, max: i32) -> i32 {
    random_double(min as f64, (max + 1) as f64) as i32
}
