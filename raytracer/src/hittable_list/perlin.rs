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
        let u = p.e[0] - p.e[0].floor();
        let v = p.e[1] - p.e[1].floor();
        let w = p.e[2] - p.e[2].floor();

        let i = p.e[0].floor() as i32;
        let j = p.e[1].floor() as i32;
        let k = p.e[2].floor() as i32;
        let mut c: [[[f64; 3]; 3]; 3] = [[[0.0, 0.0, 0.0]; 3]; 3];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[(self.x[(i as usize + di) & 255]
                        ^ self.y[(j as usize + dj) & 255]
                        ^ self.z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }
        Self::trilinear_interp(&c, u, v, w)
    }
    fn trilinear_interp(c: &[[[f64; 3]; 3]; 3], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
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
