/*use crate::*;

use std::cmp::Ordering;
use std::option::Option;

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box_: Aabb,
}
impl BvhNode {
    pub fn new(
        src_objects: &[Arc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
        let mut i = start;
        loop {
            objects.push(src_objects[i].clone());
            i += 1;
            if i == end {
                break;
            }
        }

        let axis = random_int_rng(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        let mut left = objects[0].clone();
        let mut right = left.clone();

        if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else if object_span != 1 {
            objects.sort_by(comparator);

            let mid = object_span / 2;
            left = Arc::new(BvhNode::new(&objects, 0, mid, time0, time1));
            right = Arc::new(BvhNode::new(&objects, mid, object_span, time0, time1));
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
                    box_: Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
                },
            },
            None => BvhNode {
                left,
                right,
                box_: Aabb::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)),
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
                    Some(y) => Some(y),
                    None => Some(x),
                }
            }
            None => self.right.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.box_)
    }
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e0 < y.min().e0 {
                    Ordering::Less
                } else if x.min().e0 > y.min().e0 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            None => Ordering::Less,
        },
        None => Ordering::Less,
    }
}
pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e1 < y.min().e1 {
                    Ordering::Less
                } else if x.min().e1 > y.min().e1 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            None => Ordering::Less,
        },
        None => Ordering::Less,
    }
}
pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    match box_a {
        Some(x) => match box_b {
            Some(y) => {
                if x.min().e2 < y.min().e2 {
                    Ordering::Less
                } else if x.min().e2 > y.min().e2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            None => Ordering::Less,
        },
        None => Ordering::Less,
    }
}*/
