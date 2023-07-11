use crate::*;

const POINT_COUNT: i32 = 256;
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec: Vec<Vec3> = Vec::new();
        for _i in 0..POINT_COUNT {
            ranvec.push(unit_vector(random_vec3_rng(-1.0, 1.0)));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.e0 - p.e0.floor();
        let v = p.e1 - p.e1.floor();
        let w = p.e2 - p.e2.floor();
        let i = p.e0.floor() as i32;
        let j = p.e1.floor() as i32;
        let k = p.e2.floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in [0, 1] {
            for dj in [0, 1] {
                for dk in [0, 1] {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i as usize) + di) & 255]
                        ^ self.perm_y[((j as usize) + dj) & 255]
                        ^ self.perm_z[((k as usize) + dk) & 255])
                        as usize];
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }
    pub fn turb(&self, p: Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
    
        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
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
pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in [0, 1] {
        for j in [0, 1] {
            for k in [0, 1] {
                let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));
                accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu))
                    * ((j as f64) * vv + ((1 - j) as f64) * (1.0 - vv))
                    * ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww))
                    * dot(c[i][j][k], weight_v);
            }
        }
    }

    accum
}
