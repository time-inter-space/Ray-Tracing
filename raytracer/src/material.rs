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
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let ret = Pair::new(
            self.albedo,
            Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere()),
        );
        if dot(ret.second.direction(), rec.normal) > 0.0 {
            Some(ret)
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(r_in.direction());
        let refracted = refract(unit_direction, rec.normal, refraction_ratio);
        let scattered = Ray::new(rec.p, refracted);
        Some(Pair::new(attenuation, scattered))
    }
}
