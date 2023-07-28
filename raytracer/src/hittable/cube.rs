use crate::*;

use image::{open, DynamicImage, GenericImageView};
use std::mem::swap;

pub struct Cube {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}
impl Cube {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Cube {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            p0.e0,
            p1.e0,
            p0.e1,
            p1.e1,
            p1.e2,
            ptr.clone(),
        )));
        sides.add(Arc::new(XYRect::new(
            p0.e0,
            p1.e0,
            p0.e1,
            p1.e1,
            p0.e2,
            ptr.clone(),
        )));

        sides.add(Arc::new(XZRect::new(
            p0.e0,
            p1.e0,
            p0.e2,
            p1.e2,
            p1.e1,
            ptr.clone(),
        )));
        sides.add(Arc::new(XZRect::new(
            p0.e0,
            p1.e0,
            p0.e2,
            p1.e2,
            p0.e1,
            ptr.clone(),
        )));

        sides.add(Arc::new(YZRect::new(
            p0.e1,
            p1.e1,
            p0.e2,
            p1.e2,
            p1.e0,
            ptr.clone(),
        )));
        sides.add(Arc::new(YZRect::new(
            p0.e1,
            p1.e1,
            p0.e2,
            p1.e2,
            p0.e0,
            ptr.clone(),
        )));

        Cube {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(self.box_min, self.box_max);
        Some(ret)
    }
}

const BYTES_PER_PIXEL: u32 = 3;
pub struct YZImageBox {
    box_min: Point3,
    box_max: Point3,
    bytes_per_scanline: u32,
    data: Vec<u8>,
    width: u32,
    height: u32,
}
impl YZImageBox {
    pub fn new(box_min: Point3, box_max: Point3, filename: &str) -> YZImageBox {
        let image_result: Result<DynamicImage, _> = open(filename);
        let mut data: Vec<u8> = Vec::new();
        let mut bytes_per_scanline = 0;
        let mut width = 0;
        let mut height = 0;
        match image_result {
            Ok(image) => {
                let dimensions = image.dimensions();
                width = dimensions.0;
                height = dimensions.1;
                bytes_per_scanline = BYTES_PER_PIXEL * width;
                for y in 0..height {
                    for x in 0..width {
                        let pixel = image.get_pixel(x, y);
                        data.push(pixel[0]);
                        data.push(pixel[1]);
                        data.push(pixel[2]);
                    }
                }
            }
            Err(_err) => {}
        }
        YZImageBox {
            box_min,
            box_max,
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
    fn check(&self, y: u32, z: u32) -> Option<HitRecord> {
        let pos = (y * self.bytes_per_scanline + z * BYTES_PER_PIXEL) as usize;
        let c = Color::new(
            self.data[pos] as f64 / 256.0,
            self.data[pos + 1] as f64 / 256.0,
            self.data[pos + 2] as f64 / 256.0,
        );
        if c.length_squared() < 0.5 {
            None
        } else {
            let mut ret = HitRecord::new(
                0.0,
                Point3::new(0.0, 0.0, 0.0),
                Arc::new(DiffuseLight::new(c)),
            );
            ret.front_face = true;
            Some(ret)
        }
    }
}
impl Hittable for YZImageBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut inv_d = 1.0 / r.direction().e0;
        let mut t0 = (self.box_min.e0 - r.origin().e0) / r.direction().e0;
        let mut t1 = (self.box_max.e0 - r.origin().e0) / r.direction().e0;
        let mut min = t_min;
        let mut max = t_max;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }
        inv_d = 1.0 / r.direction().e1;
        t0 = (self.box_min.e1 - r.origin().e1) / r.direction().e1;
        t1 = (self.box_max.e1 - r.origin().e1) / r.direction().e1;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }
        inv_d = 1.0 / r.direction().e2;
        t0 = (self.box_min.e2 - r.origin().e2) / r.direction().e2;
        t1 = (self.box_max.e2 - r.origin().e2) / r.direction().e2;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        min = min.max(t0);
        max = max.min(t1);
        if max <= min {
            return None;
        }

        let p0 = r.at(min);
        let p1 = r.at(max);
        let mut y0 = (p0.e1 - self.box_max.e1) / (self.box_min.e1 - self.box_max.e1);
        let mut z0 = (p0.e2 - self.box_max.e2) / (self.box_min.e2 - self.box_max.e2);
        let ymin = (clamp(y0, 0.0, 0.999) * self.height as f64) as u32;
        let ymax = (clamp(
            (p1.e1 - self.box_max.e1) / (self.box_min.e1 - self.box_max.e1),
            0.0,
            0.999,
        ) * self.height as f64) as u32;
        let mut z = (clamp(z0, 0.0, 0.999) * self.width as f64) as u32;
        let zmax = (clamp(
            (p1.e2 - self.box_max.e2) / (self.box_min.e2 - self.box_max.e2),
            0.0,
            0.999,
        ) * self.width as f64) as u32;
        y0 *= self.height as f64;
        z0 *= self.width as f64;
        let mut y1 = (p1.e1 - self.box_max.e1) / (self.box_min.e1 - self.box_max.e1);
        let mut z1 = (p1.e2 - self.box_max.e2) / (self.box_min.e2 - self.box_max.e2);
        y1 *= self.height as f64;
        z1 *= self.width as f64;
        y1 -= y0;
        z1 -= z0;
        if p0.e1 > p1.e1 {
            for y in ymin..(ymax + 1) {
                match self.check(y, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
                if p0.e2 > p1.e2 {
                    while z < zmax
                        && (((y + 1) as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * (((y + 1) as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z += 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                } else {
                    while z > zmax
                        && (((y + 1) as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * (((y + 1) as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z -= 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                }
            }
        } else {
            for y in (ymax..(ymin + 1)).rev() {
                match self.check(y, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
                if p0.e2 > p1.e2 {
                    while z < zmax
                        && ((y as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * ((y as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z += 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                } else {
                    while z > zmax
                        && ((y as f64 - y0) * z1 - (z as f64 - z0) * y1)
                            * ((y as f64 - y0) * z1 - ((z + 1) as f64 - z0) * y1)
                            > std::f64::EPSILON
                    {
                        z -= 1;
                        match self.check(y, z) {
                            Some(ret) => {
                                return Some(ret);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        if p0.e2 < p1.e2 {
            while z < zmax {
                z += 1;
                match self.check(ymax, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
            }
        } else {
            while z > zmax {
                z -= 1;
                match self.check(ymax, z) {
                    Some(ret) => {
                        return Some(ret);
                    }
                    None => {}
                }
            }
        }
        None
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(self.box_min, self.box_max);
        Some(ret)
    }
}
