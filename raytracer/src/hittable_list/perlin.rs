use crate::rtweekend::random_double_01;
use crate::rtweekend::random_int;
use crate::rtweekend::vec3::Point3;

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    x: [i32; POINT_COUNT],
    y: [i32; POINT_COUNT],
    z: [i32; POINT_COUNT],
    randfloat: [f64; POINT_COUNT],
}
impl Perlin {
    pub fn build_perlin() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for item in randfloat.iter_mut().take(POINT_COUNT) {
            *item = random_double_01();
        }
        Perlin {
            x: Self::perlin_generate_perm(),
            y: Self::perlin_generate_perm(),
            z: Self::perlin_generate_perm(),
            randfloat,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4_f64 * p.e[0]) as i32) & 255;
        let j = ((4_f64 * p.e[1]) as i32) & 255;
        let k = ((4_f64 * p.e[2]) as i32) & 255;

        self.randfloat[(self.x[i as usize] ^ self.y[j as usize] ^ self.z[k as usize]) as usize]
    }
    fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
        for i in 0..n {
            let target = random_int(0, i as i32);
            p.swap(i, target as usize);
        }
    }
    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as i32;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }
}
