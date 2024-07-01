use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::ops::{Add,Sub,Mul,Div,Neg};

pub struct Vec3 {
    e: [f64; 3],
}
pub impl Add for Vec3{
    type Output=Vec3;
    fn add(self,other:Vec3)->Vec3{
        Vec3{
            e:[self.e[0]+other.e[0],self.e[1]+other.e[1],self.e[2]+other.e[2]],
        }
    }
}
pub impl Sub for Vec3{
    type Output=Vec3;
    fn sub(self,other:Vec3)->Vec3{
        Vec3{
            e:[self.e[0]-other.e[0],self.e[1]-other.e[1],self.e[2]-other.e[2]],
        }
    }
}
pub impl Mul<Vec3> for Vec3{
    type Output=Vec3;
    fn mul(self,other:Vec3)->Vec3{
        Vec3{
            e:[self.e[0]*other.e[0],self.e[1]*other.e[1],self.e[2]*other.e[2]],
        }
    }
}
pub impl Mul<f64> for Vec3{
    type Output=Vec3;
    fn mul(self,other:f64)->Vec3{
        Vec3{
            e:[self.e[0]*other,self.e[1]*other,self.e[2]*other],
        }
    }
}
pub impl Div<f64> for Vec3{
    type Output=Vec3;
    fn div(self,other:f64)->Vec3{
        self*(1.0/other)
    }
}
pub impl Neg for Vec3{
    type Output=Vec3;
    fn neg(self)->Vec3{
        Vec3{
            e:[-self.e[0],-self.e[1],-self.e[2]],
        }
    }
}
pub impl Vec3 {
    fn _x(&self) -> f64 {
        self.e[0]
    }
    fn _y(&self) -> f64 {
        self.e[1]
    }
    fn _z(&self) -> f64 {
        self.e[2]
    }
    fn fushu(&self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
    fn sq_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    fn length(&self) -> f64 {
        self.sq_length().sqrt()
    }
    fn _self_add(&mut self, x: Self) {
        self.e[0] += x.e[0];
        self.e[1] += x.e[1];
        self.e[2] += x.e[2];
    }
    fn _self_mul(&mut self, x: f64) {
        self.e[0] *= x;
        self.e[1] *= x;
        self.e[2] *= x;
    }
    fn _self_div(&mut self, x: f64) {
        self.e[0] /= x;
        self.e[1] /= x;
        self.e[2] /= x;
    }
    fn add(x: &Self, y: &Self) -> Self {
        Self {
            e: [x.e[0] + y.e[0], x.e[1] + y.e[1], x.e[2] + y.e[2]],
        }
    }
    fn del(x: &Self, y: &Self) -> Self {
        Self {
            e: [x.e[0] - y.e[0], x.e[1] - y.e[1], x.e[2] - y.e[2]],
        }
    }
    fn mul(x: &Self, y: f64) -> Self {
        Self {
            e: [x.e[0] * y, x.e[1] * y, x.e[2] * y],
        }
    }
    fn div(x: &Self, y: f64) -> Self {
        Self {
            e: [x.e[0] / y, x.e[1] / y, x.e[2] / y],
        }
    }
    fn dot(x: &Self, y: &Self) -> f64 {
        x.e[0] * y.e[0] + x.e[1] * y.e[1] + x.e[2] * y.e[2]
    }
    fn _cross(x: &Self, y: &Self) -> Self {
        Self {
            e: [
                x.e[1] * y.e[2] - x.e[2] * y.e[1],
                x.e[2] * y.e[0] - x.e[0] * y.e[2],
                x.e[0] * y.e[1] - x.e[1] * y.e[0],
            ],
        }
    }
    fn unit_vector(x: &Self) -> Self {
        let y = x.length();
        Self::div(x, y)
    }
    fn output(x: &Self, file: &mut File) {
        let r: u32 = (x.e[0] * 255.999) as u32;
        let g: u32 = (x.e[1] * 255.999) as u32;
        let b: u32 = (x.e[2] * 255.999) as u32;
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
    fn clone(&self) -> Self {
        Self {
            e: [self.e[0], self.e[1], self.e[2]],
        }
    }
}
pub type Color = Vec3;
pub type Point3 = Vec3;