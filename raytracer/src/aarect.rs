use crate::*;

use std::option::Option;

pub struct XYRect {
    mp: Rc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}
impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Rc<dyn Material>) -> XYRect {
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
        let mut rec = HitRecord::new(t, r.at(t), &self.mp.clone());
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
