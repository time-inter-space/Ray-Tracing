use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}
