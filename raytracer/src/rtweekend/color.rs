use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Color;
use std::fs::File;
use std::io::Write;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
pub fn write_color(pixel_color: &Color, file: &mut File) {
    let mut r = pixel_color.e[0];
    let mut g = pixel_color.e[1];
    let mut b = pixel_color.e[2];

    if r != r {
        r = 0.0;
    }
    if g != g {
        g = 0.0;
    }
    if b != b {
        b = 0.0;
    }

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval {
        min: 0.000,
        max: 0.999,
    };
    let rbyte = (256.0 * intensity.clamp(r)) as u32;
    let gbyte = (256.0 * intensity.clamp(g)) as u32;
    let bbyte = (256.0 * intensity.clamp(b)) as u32;
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
