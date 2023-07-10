/*use crate::*;

use std::cmp::Ordering;
use std::option::Option;
use std::rc::Rc;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub box_: AABB,
}
impl BvhNode {
    pub fn new(
        src_objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects = src_objects.clone();

        let axis = random_int_rng(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        let mut left = objects[start].clone();
        let mut right = left.clone();

        if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else if object_span != 1 {
            objects.sort_by(comparator);

            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::new(&objects, start, mid, time0, time1));
            right = Rc::new(BvhNode::new(&objects, mid, end, time0, time1));
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        match box_left {
            Some(x) => match box_right {
                Some(y) => {
                    let box_ = surrounding_box(x, y);
                    BvhNode { left, right, box_ }
                }
                None => BvhNode {
                    left,
                    right,
                    box_: AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
                },
            },
            None => BvhNode {
                left,
                right,
                box_: AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
            },
        }
    }
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box_.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        match hit_left {
            Some(x) => {
                let hit_right = self.right.hit(r, t_min, x.t);
                match hit_right {
                    Some(y) => {
                        return Some(y);
                    }
                    None => {
                        return Some(x);
                    }
                }
            }
            None => {
                return self.right.hit(r, t_min, t_max);
            }
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.box_)
    }
}
pub fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e0 < y.min().e0 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            None => {
                return Ordering::Less;
            }
        },
        None => {
            return Ordering::Less;
        }
    }
}
pub fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e1 < y.min().e1 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            None => {
                return Ordering::Less;
            }
        },
        None => {
            return Ordering::Less;
        }
    }
}
pub fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e2 < y.min().e2 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            None => {
                return Ordering::Less;
            }
        },
        None => {
            return Ordering::Less;
        }
    }
}*/
