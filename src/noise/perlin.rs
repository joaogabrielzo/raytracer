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
        let i = (4. * p.x) as usize & 255;
        let j = (4. * p.y) as usize & 255;
        let k = (4. * p.z) as usize & 255;

        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; point_count];

        (0..point_count).for_each(|i| p[i] = i as i32);

        Self::permute(&mut p, point_count);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        (0..n).rev().for_each(|i| {
            let target = random_int_rng(0, i as i32);
            p.swap(i, target as usize);
        })
    }
}
