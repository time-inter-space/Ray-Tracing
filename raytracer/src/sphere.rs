use crate::*;

use std::option::Option;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
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
}
/*impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}*/
pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.e1).acos();
    let phi = (-p.e2).atan2(p.e0) + std::f64::consts::PI;

    *u = phi / (2.0 * std::f64::consts::PI);
    *v = theta / std::f64::consts::PI;
}
