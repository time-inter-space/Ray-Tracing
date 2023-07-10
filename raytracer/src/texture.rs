use crate::*;

use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    pub color_value: Color,
}
impl SolidColor {
    pub fn new(color_value: Color) -> SolidColor {
        SolidColor { color_value }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.e0).sin() * (10.0 * p.e1).sin() * (10.0 * p.e2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
