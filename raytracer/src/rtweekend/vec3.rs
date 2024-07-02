use std::fs::File;
use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl Vec3 {
    pub fn sq_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        self.sq_length().sqrt()
    }
    pub fn dot(x: &Self, y: &Self) -> f64 {
        x.e[0] * y.e[0] + x.e[1] * y.e[1] + x.e[2] * y.e[2]
    }
    pub fn _cross(x: &Self, y: &Self) -> Self {
        Self {
            e: [
                x.e[1] * y.e[2] - x.e[2] * y.e[1],
                x.e[2] * y.e[0] - x.e[0] * y.e[2],
                x.e[0] * y.e[1] - x.e[1] * y.e[0],
            ],
        }
    }
    pub fn unit_vector(x: Self) -> Self {
        let y = x.length();
        x / y
    }
    pub fn _output(x: &Self, file: &mut File) {
        let r: u32 = (x.e[0] * 255.999) as u32;
        let g: u32 = (x.e[1] * 255.999) as u32;
        let b: u32 = (x.e[2] * 255.999) as u32;
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
    pub fn clone(&self) -> Self {
        Self {
            e: [self.e[0], self.e[1], self.e[2]],
        }
    }
    pub fn new() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}
pub type Color = Vec3;
pub type Point3 = Vec3;
