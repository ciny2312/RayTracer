use crate::rtweekend::random_double;
use crate::rtweekend::random_int;

const:i32 point_count=256;
struct Perlin{
    x:[i32,point_count],
    y:[i32,point_count],
    z:[i32,point_count],
    randfloat:[f64,point_count],
}
impl Perlin{
    fn build_perlin()->Self{
        let randfloat:[f64,point_count];
        for i in 0..point_count{
            randfloat[i]=random_double();
        }
        Perlin{
            x:Self::perlin_generate_perm(),
            y:Self::perlin_generate_perm(),
            z:Self::perlin_generate_perm(),
            randfloat,
        }
    }
    pub fn noise(p:&Point3)->f64{
        let i=((4 as f64*p.e[0]) as i32)&255;
        let j=((4 as f64*p.e[1]) as i32)&255;
        let k=((4 as f64*p.e[2]) as i32)&255;

        return randfloat[x[i] ^ y[j] ^ z[k]];
    }
    fn perlin_generate_perm()->[i32,point_count]{
        let p=[i32,point_count];
        for i in 0..point_count{
            p[i]=i;
        }
        Self::permute(&mut p,point_count);
        p
    }
    fn permute(p:&mut [i32,point_count],n:i32){
        for i in 0..n{
            let target=random_int(0,i);
            p[i].swap(p[target as usize]);
        }
    }
}