/*use crate::*;

use std::option::Option;

pub struct XYRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}
impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> XYRect {
        XYRect {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}
impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().e2) / r.direction().e2;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().e0 + t * r.direction().e0;
        let y = r.origin().e1 + t * r.direction().e1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), self.mp.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        Some(ret)
    }
}

#[derive(Clone)]
pub struct XZRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}
impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().e1) / r.direction().e1;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().e0 + t * r.direction().e0;
        let z = r.origin().e2 + t * r.direction().e2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), self.mp.clone());
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        Some(ret)
    }
    fn pdf_value(&self, origin: Point3, v: Vec3) -> f64 {
        let rec = self.hit(&Ray::new(origin, v, 0.0), 0.001, std::f64::INFINITY);
        match rec {
            Some(x) => {
                let area = (self.x1 - self.x0) * (self.z1 - self.z0);
                let distance_squared = x.t * x.t * v.length_squared();
                let cosine = (dot(v, x.normal) / v.length()).abs();

                distance_squared / (cosine * area)
            }
            None => 0.0,
        }
    }
    fn random(&self, origin: Point3) -> Vec3 {
        let random_point = Point3::new(
            random_double_rng(self.x0, self.x1),
            self.k,
            random_double_rng(self.z0, self.z1),
        );
        random_point - origin
    }
}

pub struct YZRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}
impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> YZRect {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}
impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().e0) / r.direction().e0;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().e1 + t * r.direction().e1;
        let z = r.origin().e2 + t * r.direction().e2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(t, r.at(t), self.mp.clone());
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        let ret = Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        Some(ret)
    }
}*/
