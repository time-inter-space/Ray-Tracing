use crate::*;

use std::f64::INFINITY;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
/*impl ConstantMedium {
    /*pub fn new_p(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_p(a)),
        }
    }*/
    pub fn new_c(b: Arc<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new_c(c)),
        }
    }
}*/
impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let option_rec1 = self.boundary.hit(r, -INFINITY, INFINITY);
        match option_rec1 {
            Some(mut rec1) => {
                let option_rec2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
                match option_rec2 {
                    Some(mut rec2) => {
                        if rec1.t < t_min {
                            rec1.t = t_min;
                        }
                        if rec2.t > t_max {
                            rec2.t = t_max;
                        }

                        if rec1.t >= rec2.t {
                            return None;
                        }

                        if rec1.t < 0.0 {
                            rec1.t = 0.0;
                        }

                        let ray_length = r.direction().length();
                        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                        let hit_distance = self.neg_inv_density * random_double().ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let t = rec1.t + hit_distance / ray_length;
                        let mut rec = HitRecord::new(t, r.at(t), self.phase_function.clone());
                        rec.normal = Vec3::new(1.0, 0.0, 0.0);
                        rec.front_face = true;

                        Some(rec)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
