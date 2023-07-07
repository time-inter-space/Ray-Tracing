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
/*impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}*/
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
/*impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}*/
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };
        let scattered = Ray::new(rec.p, direction);
        Some(Pair::new(attenuation, scattered))
    }
}
