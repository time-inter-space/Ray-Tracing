use crate::*;

//use image::{open, DynamicImage, GenericImageView};
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
/*impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2)),
        }
    }
}*/
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

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
/*impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}*/
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.e2 + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

const BYTES_PER_PIXEL: u32 = 3;
pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}
/*impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let image_result: Result<DynamicImage, _> = open(filename);
        let mut data: Vec<u8> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        let mut bytes_per_scanline = 0;

        match image_result {
            Ok(image) => {
                let dimensions = image.dimensions();
                width = dimensions.0;
                height = dimensions.1;
                bytes_per_scanline = BYTES_PER_PIXEL * width;
                for y in 0..height {
                    for x in 0..dimensions.0 {
                        let pixel = image.get_pixel(x, y);
                        data.push(pixel[0]);
                        data.push(pixel[1]);
                        data.push(pixel[2]);
                    }
                }
            }
            Err(_err) => {}
        }
        ImageTexture {
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
}*/
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: Vec3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * (self.width as f64)) as u32;
        let mut j = (v * (self.height as f64)) as u32;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pos = (j * self.bytes_per_scanline + i * BYTES_PER_PIXEL) as usize;
        Color::new(
            color_scale * (self.data[pos] as f64),
            color_scale * (self.data[pos + 1] as f64),
            color_scale * (self.data[pos + 2] as f64),
        )
    }
}
