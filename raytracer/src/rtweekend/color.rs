use std::fs::File;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Color;
use std::io::Write;

pub fn write_color(pixel_color:&Color,file:&mut File){
    let r=pixel_color.e[0];
    let g=pixel_color.e[1];
    let b=pixel_color.e[2];
    let intensity=Interval{
        min:0.000,
        max:0.999,
    };
    let rbyte=(256.0*intensity.clamp(r)) as u32;
    let gbyte=(256.0*intensity.clamp(g)) as u32;
    let bbyte=(256.0*intensity.clamp(b)) as u32;
    writeln!(file, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}