use crate::*;

use std::option::Option;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>>;
}

pub struct Lambertian {
    albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>> {
        if dot(r_in.direction(), rec.normal) > 0.0 {
            return None;
        }
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let ret = Pair::new(self.albedo, Ray::new(rec.p, scatter_direction));
        Some(ret)
    }
}
impl Lambertian {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Lambertian {
        Lambertian {
            albedo: Color::new(e0, e1, e2),
        }
    }
}

pub struct Metal {
    albedo: Color,
}
impl Metal {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Metal {
        Metal {
            albedo: Color::new(e0, e1, e2),
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let ret = Pair::new(self.albedo, Ray::new(rec.p, reflected));
        if dot(ret.second.direction(), rec.normal) > 0.0 {
            Some(ret)
        } else {
            None
        }
    }
}
