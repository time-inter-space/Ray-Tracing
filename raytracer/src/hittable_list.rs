use crate::*;

use std::option::Option;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let temp_rec = object.hit(r, t_min, closest_so_far);
            match temp_rec {
                Some(x) => {
                    rec = Some(x.clone());
                    closest_so_far = x.t;
                }
                None => {}
            }
        }
        rec
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = None;
        for object in &self.objects {
            let temp_box = object.bounding_box(time0, time1);
            match temp_box {
                Some(x) => match output_box {
                    Some(y) => {
                        output_box = Some(surrounding_box(y, x));
                    }
                    None => {
                        output_box = Some(x);
                    }
                },
                None => {
                    return None;
                }
            }
        }
        output_box
    }
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    /*pub fn clear(&mut self) {
        self.objects.clear();
    }*/
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}
