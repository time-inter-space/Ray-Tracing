use crate::*;

use std::option::Option;

pub trait Material: Send + Sync {
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Pair<Color, Pair<Ray, f64>>> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Pair<Ray, f64>>> {
        let direction = random_in_hemisphere(rec.normal);
        let scattered = Ray::new(rec.p, unit_vector(direction), r_in.time());
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = 0.5 / std::f64::consts::PI;
        let ret = Pair::new(alb, Pair::new(scattered, pdf));
        Some(ret)
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(rec.normal, unit_vector(scattered.direction()));
        if cosine < 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }
}
impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
}

/*pub struct Metal {
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
            Ray::new(
                rec.p,
                reflected + self.fuzz * random_in_unit_sphere(),
                r_in.time(),
            ),
        );
        if dot(ret.second.direction(), rec.normal) > 0.0 {
            Some(ret)
        } else {
            None
        }
    }
}*/

/*pub struct Dielectric {
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
        let scattered = Ray::new(rec.p, direction, r_in.time());
        Some(Pair::new(attenuation, scattered))
    }
}*/

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<Pair<Color, Pair<Ray, f64>>> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

/*pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new_c(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
    pub fn new_p(a: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo: a }
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Pair<Color, Ray>> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some(Pair::new(attenuation, scattered))
    }
}*/
