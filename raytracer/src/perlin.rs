use crate::*;

const POINT_COUNT: i32 = 256;
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat: Vec<f64> = Vec::new();
        for _i in 0..POINT_COUNT {
            ranfloat.push(random_double());
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        Perlin {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let i = (((4.0 * p.e0) as i32) & 255) as usize;
        let j = (((4.0 * p.e1) as i32) & 255) as usize;
        let k = (((4.0 * p.e2) as i32) & 255) as usize;

        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}
pub fn perlin_generate_perm() -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();

    for i in 0..POINT_COUNT {
        ret.push(i);
    }

    permute(&mut ret, POINT_COUNT);

    ret
}
pub fn permute(p: &mut [i32], n: i32) {
    for i in (1..n).rev() {
        let target = random_int_rng(0, i);
        p.swap(i as usize, target as usize);
    }
}
