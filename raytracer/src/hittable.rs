use crate::*;

use std::f64::INFINITY;
use std::option::Option;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(t: f64, p: Point3, mat_ptr: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr,
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}
impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { ptr, offset }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        let rec = self.ptr.hit(&moved_r, t_min, t_max);
        match rec {
            Some(x) => {
                let mut ret = x;
                ret.p = ret.p + self.offset;
                ret.set_face_normal(&moved_r, ret.normal);
                Some(ret)
            }
            None => None,
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let output_box = self.ptr.bounding_box(time0, time1);
        match output_box {
            Some(x) => {
                let ret = Aabb::new(x.min() + self.offset, x.max() + self.offset);
                Some(ret)
            }
            None => None,
        }
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}
impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = p.bounding_box(0.0, 1.0);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        match bbox {
            Some(b) => {
                for i in [0, 1] {
                    for j in [0, 1] {
                        for k in [0, 1] {
                            let x = (i as f64) * b.max().e0 + ((1 - i) as f64) * b.min().e0;
                            let y = (j as f64) * b.max().e1 + ((1 - j) as f64) * b.min().e1;
                            let z = (k as f64) * b.max().e2 + ((1 - k) as f64) * b.min().e2;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(newx, y, newz);

                            min.e0 = min.e0.min(tester.e0);
                            min.e1 = min.e1.min(tester.e1);
                            min.e2 = min.e2.min(tester.e2);
                            max.e0 = max.e0.max(tester.e0);
                            max.e1 = max.e1.max(tester.e1);
                            max.e2 = max.e2.max(tester.e2);
                        }
                    }
                }
            }
            None => {}
        }
        bbox = Some(Aabb::new(min, max));
        RotateY {
            ptr: p,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.e0 = self.cos_theta * r.origin().e0 - self.sin_theta * r.origin().e2;
        origin.e2 = self.sin_theta * r.origin().e0 + self.cos_theta * r.origin().e2;

        direction.e0 = self.cos_theta * r.direction().e0 - self.sin_theta * r.direction().e2;
        direction.e2 = self.sin_theta * r.direction().e0 + self.cos_theta * r.direction().e2;

        let rotated_r = Ray::new(origin, direction, r.time());

        let rec = self.ptr.hit(&rotated_r, t_min, t_max);
        match rec {
            Some(x) => {
                let mut ret = x;
                let mut p = ret.p;
                let mut normal = ret.normal;

                p.e0 = self.cos_theta * ret.p.e0 + self.sin_theta * ret.p.e2;
                p.e2 = -self.sin_theta * ret.p.e0 + self.cos_theta * ret.p.e2;

                normal.e0 = self.cos_theta * ret.normal.e0 + self.sin_theta * ret.normal.e2;
                normal.e2 = -self.sin_theta * ret.normal.e0 + self.cos_theta * ret.normal.e2;

                ret.p = p;
                ret.set_face_normal(&rotated_r, normal);

                Some(ret)
            }
            None => None,
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}
