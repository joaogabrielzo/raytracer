use crate::{random, random_int_rng, vector::Point};

#[derive(Clone)]
pub struct Perlin {
    pub point_count: usize,
    pub ranfloat: Vec<f32>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count = 256;
        let mut ranfloat: Vec<f32> = vec![0.0; point_count];

        (0..point_count).for_each(|i| ranfloat[i] = random());

        let perm_x = Self::perlin_generate_perm(point_count);
        let perm_y = Self::perlin_generate_perm(point_count);
        let perm_z = Self::perlin_generate_perm(point_count);

        Self {
            point_count,
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f32 {
        let mut u = p.x - p.x.floor() as f32;
        let mut v = p.y - p.y.floor() as f32;
        let mut w = p.z - p.z.floor() as f32;

        u = u * u * (3. - 2. * u);
        v = v * v * (3. - 2. * v);
        w = w * w * (3. - 2. * w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; 2]; 2]; 2];

        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
                    let idx = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];
                    c[di][dj][dk] = self.ranfloat[idx as usize];
                });
            });
        });

        Self::trilinear_interpolation(c, u, v, w)
    }

    fn trilinear_interpolation(c: Vec<Vec<Vec<f32>>>, u: f32, v: f32, w: f32) -> f32 {
        let mut accum = 0.0;

        (0..2).for_each(|i| {
            (0..2).for_each(|j| {
                (0..2).for_each(|k| {
                    let if32 = i as f32;
                    let jf = j as f32;
                    let kf = k as f32;
                    accum += (if32 * u + (1. - if32) * (1. - u))
                        * (jf * u + (1. - jf) * (1. - v))
                        * (kf * u + (1. - kf) * (1. - w))
                        * c[i][j][k];
                });
            });
        });

        accum
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; point_count];

        (0..point_count).for_each(|i| p[i] = i as i32);

        Self::permute(&mut p, point_count);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        (0..n).rev().for_each(|i| {
            let target = random_int_rng(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        })
    }
}
