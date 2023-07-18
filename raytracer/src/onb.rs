use crate::*;

pub struct Onb {
    pub axis: [Vec3; 3],
}
impl Onb {
    pub fn new() -> Onb {
        Onb {
            axis: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ],
        }
    }
    /*pub fn u(&self) -> Vec3 {
        self.axis[0]
    }*/
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    /*pub fn local_double(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.axis[0] + b * self.axis[1] + c * self.axis[2]
    }*/
    pub fn local_vec3(&self, a: Vec3) -> Vec3 {
        a.e0 * self.axis[0] + a.e1 * self.axis[1] + a.e2 * self.axis[2]
    }

    pub fn build_from_w(&mut self, n: Vec3) {
        self.axis[2] = unit_vector(n);
        let a = if self.w().e0.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        self.axis[1] = unit_vector(cross(self.w(), a));
        self.axis[0] = cross(self.w(), self.v());
    }
}
