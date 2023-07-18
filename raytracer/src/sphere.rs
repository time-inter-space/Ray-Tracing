use crate::*;

use std::option::Option;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
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
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        let rec = self.hit(&Ray::new(o, v, 0.0), 0.001, std::f64::INFINITY);
        match rec {
            Some(_x) => {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
                let solid_angle = 2.0 * std::f64::consts::PI * (1.0 - cos_theta_max);
                1.0 / solid_angle
            }
            None => 0.0,
        }
    }
    fn random(&self, o: Point3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let mut uvw = Onb::new();
        uvw.build_from_w(direction);
        uvw.local_vec3(random_to_sphere(self.radius, distance_squared))
    }
}
pub fn get_sphere_uv(p: Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.e1).acos();
    let phi = (-p.e2).atan2(p.e0) + std::f64::consts::PI;

    *u = phi / (2.0 * std::f64::consts::PI);
    *v = theta / std::f64::consts::PI;
}
