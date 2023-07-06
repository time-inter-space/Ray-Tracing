use crate::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}
pub type Point3 = Vec3;
pub type Color = Vec3;
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.e0 + other.e0, self.e1 + other.e1, self.e2 + other.e2)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.e0 - other.e0, self.e1 - other.e1, self.e2 - other.e2)
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.e0, -self.e1, -self.e2)
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.e0 * other, self.e1 * other, self.e2 * other)
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e0, self * other.e1, self * other.e2)
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.e0 / other, self.e1 / other, self.e2 / other)
    }
}
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e0, e1, e2 }
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}
/*pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e1 * v.e2 - u.e2 * v.e1,
        u.e2 * v.e0 - u.e0 * v.e2,
        u.e0 * v.e1 - u.e1 * v.e0,
    )
}*/
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
/*pub fn random_vec3() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}*/
pub fn random_vec3_rng(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_double_rng(min, max),
        random_double_rng(min, max),
        random_double_rng(min, max),
    )
}
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_rng(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}
/*pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}*/
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
