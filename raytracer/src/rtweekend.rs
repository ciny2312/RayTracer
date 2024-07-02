use rand::Rng;
pub mod ray;
pub mod color;
pub mod vec3;
pub mod interval;
pub const INF: f64 = 1e18;
//const PI: f64 = 3.1415926535897932385;

//fn _degree_to_radians(degree: f64) -> f64 {
//    degree * PI / 180.0
//}
pub fn random_double_01()->f64{
    let mut rng=rand::thread_rng();
    rng.gen_range(0.0..1.0)
}
pub fn _random_double(min:f64,max:f64)->f64{
    min+random_double_01()*(max-min)
}