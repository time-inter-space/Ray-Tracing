use crate::*;

use std::option::Option;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
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
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[random_int_rng(0, int_size - 1) as usize].random(o)
    }
}
