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
        let mut u = p.e0 - p.e0.floor();
        let mut v = p.e1 - p.e1.floor();
        let mut w = p.e2 - p.e2.floor();
        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.e0.floor() as i32;
        let j = p.e1.floor() as i32;
        let k = p.e2.floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in [0, 1] {
            for dj in [0, 1] {
                for dk in [0, 1] {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i as usize) + di) & 255]
                        ^ self.perm_y[((j as usize) + dj) & 255]
                        ^ self.perm_z[((k as usize) + dk) & 255])
                        as usize];
                }
            }
        }

        trilinear_interp(c, u, v, w)
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
pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in [0, 1] {
        for j in [0, 1] {
            for k in [0, 1] {
                accum += ((i as f64) * u + ((1 - i) as f64) * (1.0 - u))
                    * ((j as f64) * v + ((1 - j) as f64) * (1.0 - v))
                    * ((k as f64) * w + ((1 - k) as f64) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    accum
}
