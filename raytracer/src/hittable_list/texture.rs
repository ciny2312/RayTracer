use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;

#[derive(Clone, Debug)]
pub enum Texture {
    SolidColor {
        albedo: Color,
    },
    CheckerTexture {
        inv_scale: f64,
        even: Box<Texture>,
        odd: Box<Texture>,
    },
}
impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::SolidColor { albedo } => *albedo,
            Texture::CheckerTexture {
                inv_scale,
                even,
                odd,
            } => {
                let x_integer = (inv_scale * p.e[0]).floor() as i32;
                let y_integer = (inv_scale * p.e[1]).floor() as i32;
                let z_integer = (inv_scale * p.e[2]).floor() as i32;

                let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
                if is_even {
                    even.value(u, v, p)
                } else {
                    odd.value(u, v, p)
                }
            }
        }
    }
}
