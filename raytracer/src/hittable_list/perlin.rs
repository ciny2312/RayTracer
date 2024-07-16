//use crate::rtweekend::random_double_01;
use crate::rtweekend::random_int;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::vec3::Vec3;

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct Perlin {
    x: [i32; POINT_COUNT],
    y: [i32; POINT_COUNT],
    z: [i32; POINT_COUNT],
    randvec: [Vec3; POINT_COUNT],
}
impl Perlin {
    pub fn _build_perlin() -> Self {
        let mut randvec = [Vec3::new(); POINT_COUNT];
        for item in randvec.iter_mut().take(POINT_COUNT) {
            *item = Vec3::unit_vector(Vec3::random(-1.0, 1.0));
        }
        Perlin {
            x: Self::_perlin_generate_perm(),
            y: Self::_perlin_generate_perm(),
            z: Self::_perlin_generate_perm(),
            randvec,
        }
    }
    fn _perlin_interp(c: &[[[Vec3; 3]; 3]; 3], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for (i, itemi) in c.iter().enumerate().take(2) {
            for (j, itemj) in itemi.iter().enumerate().take(2) {
                for (k, itemk) in itemj.iter().enumerate().take(2) {
                    let weight_v = Vec3 {
                        e: [u - i as f64, v - j as f64, w - k as f64],
                    };
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * Vec3::dot(itemk, &weight_v);
                }
            }
        }
        accum
    }
    fn _permute(p: &mut [i32; POINT_COUNT], n: usize) {
        for i in 0..n {
            let target = random_int(0, i as i32);
            p.swap(i, target as usize);
        }
    }
    fn _perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as i32;
        }
        Self::_permute(&mut p, POINT_COUNT);
        p
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.e[0] - p.e[0].floor();
        let v = p.e[1] - p.e[1].floor();
        let w = p.e[2] - p.e[2].floor();

        let i = p.e[0].floor() as i32;
        let j = p.e[1].floor() as i32;
        let k = p.e[2].floor() as i32;
        let mut c: [[[Vec3; 3]; 3]; 3] = [[[Vec3::new(); 3]; 3]; 3];

        for (di, itemi) in c.iter_mut().enumerate().take(2) {
            for (dj, itemj) in itemi.iter_mut().enumerate().take(2) {
                for (dk, itemk) in itemj.iter_mut().enumerate().take(2) {
                    *itemk = self.randvec[(self.x[(i as usize + di) & 255]
                        ^ self.y[(j as usize + dj) & 255]
                        ^ self.z[(k as usize + dk) & 255])
                        as usize];
                }
            }
        }
        Self::_perlin_interp(&c, u, v, w)
    }
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += self.noise(&temp_p) * weight;
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}
