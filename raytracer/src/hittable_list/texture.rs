use crate::rtw_image::RtwImage;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Color;
use crate::rtweekend::vec3::Point3;

use crate::hittable_list::perlin::Perlin;

#[derive(Clone, Debug)]
pub enum Texture {
    SolidColor { albedo: Color },
    /*    Checkertexture {
        inv_scale: f64,
        even: Box<Texture>,
        odd: Box<Texture>,
    },*/
    _Imagetexture { image: Box<RtwImage> },
    _Noisetexture { noise: Box<Perlin>, scale: f64 },
}
impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::SolidColor { albedo } => *albedo,
            /*    Texture::Checkertexture {
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
            }*/
            Texture::_Imagetexture { image } => {
                let interval = Interval { min: 0.0, max: 1.0 };
                let u = interval.clamp(u);
                let v = 1.0 - interval.clamp(v);
                let i = (u * image.width as f64) as u32;
                let j = (v * image.height as f64) as u32;
                Color {
                    e: image.pixel_data(i, j),
                }
            }
            Texture::_Noisetexture { noise, scale } => {
                Color { e: [0.5, 0.5, 0.5] }
                    * (1.0 + (scale * p.e[2] + 10.0 * noise.turb(p, 7)).sin())
            }
        }
    }
}
