use crate::*;

use image::{open, DynamicImage, GenericImageView};
use std::f64::consts::PI;
use std::option::Option;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord::new(t, p, self.mat_ptr.clone());
        rec.set_face_normal(r, outward_normal);
        get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);

        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let rec = self.hit(&Ray::new(o, v, 0.0), 0.001, std::f64::INFINITY);
        match rec {
            Some(_x) => {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            None => 0.0,
        }
    }
    fn random(&self, o: Point3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw = Onb::new();
        uvw.build_from_w(direction);
        uvw.local_vec3(random_to_sphere(self.radius, distance_squared))
    }
}
pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.e1).acos();
    let phi = (-p.e2).atan2(p.e0) + PI;

    *u = phi / (2.0 * PI);
    *v = theta / PI;
}

const BYTES_PER_PIXEL: u32 = 3;
pub struct NormalMappingSphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
    scale: i32,
    bytes_per_scanline: u32,
    data: Vec<u8>,
    width: u32,
    height: u32,
}
impl NormalMappingSphere {
    pub fn new(
        center: Point3,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
        filename: &str,
        scale: i32,
    ) -> NormalMappingSphere {
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
        NormalMappingSphere {
            center,
            radius,
            mat_ptr,
            scale,
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
}
impl Hittable for NormalMappingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let mut outward_normal = (p - self.center) / self.radius;
        let mut rec = HitRecord::new(t, p, self.mat_ptr.clone());
        get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);

        let z = outward_normal;
        let y = unit_vector(cross(Vec3::new(1.0, 0.0, 0.0), z));
        let x = cross(y, z);
        let mut j = ((-outward_normal.e2).acos() / PI * self.scale as f64 * self.height as f64)
            as u32
            % self.height as u32;
        j = self.height - j - 1;
        let i = (((-outward_normal.e0).atan2(outward_normal.e1) + PI) / (2.0 * PI)
            * self.scale as f64
            * self.width as f64) as u32
            % self.width as u32;
        let pos = (j * self.bytes_per_scanline + i * BYTES_PER_PIXEL) as usize;
        let v = unit_vector(
            (Vec3::new(
                self.data[pos] as f64,
                self.data[pos + 1] as f64,
                self.data[pos + 2] as f64,
            ) / 256.0
                - Vec3::new(0.5, 0.5, 0.5))
                * 2.0,
        );
        outward_normal = x * v.e0 + y * v.e1 + z * v.e2;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let rec = self.hit(&Ray::new(o, v, 0.0), 0.001, std::f64::INFINITY);
        match rec {
            Some(_x) => {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
                let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            None => 0.0,
        }
    }
    fn random(&self, o: Point3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw = Onb::new();
        uvw.build_from_w(direction);
        uvw.local_vec3(random_to_sphere(self.radius, distance_squared))
    }
}
