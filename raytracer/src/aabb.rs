use crate::*;

//use std::mem::swap;

#[derive(Clone, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}
impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Aabb {
        Aabb { minimum, maximum }
    }
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }
    /*pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let mut inv_d = 1.0 / r.direction().e0;
        let mut t0 = (self.minimum.e0 - r.origin().e0) / r.direction().e0;
        let mut t1 = (self.minimum.e0 - r.origin().e0) / r.direction().e0;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        inv_d = 1.0 / r.direction().e1;
        t0 = (self.minimum.e1 - r.origin().e1) / r.direction().e1;
        t1 = (self.minimum.e1 - r.origin().e1) / r.direction().e1;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        inv_d = 1.0 / r.direction().e2;
        t0 = (self.minimum.e2 - r.origin().e2) / r.direction().e2;
        t1 = (self.minimum.e2 - r.origin().e2) / r.direction().e2;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }
        true
    }*/
}
pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point3::new(
        box0.min().e0.min(box1.min().e0),
        box0.min().e1.min(box1.min().e1),
        box0.min().e2.min(box1.min().e2),
    );
    let big = Point3::new(
        box0.max().e0.max(box1.max().e0),
        box0.max().e1.max(box1.max().e1),
        box0.max().e2.max(box1.max().e2),
    );
    Aabb::new(small, big)
}
