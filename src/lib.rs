use rand::Rng;
use vector::Vector3;

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod noise;
pub mod ray;
pub mod shape;
pub mod texture;
pub mod vector;

pub fn dot(fst: &Vector3, snd: &Vector3) -> f32 {
    fst.dot(snd)
}

pub fn random_rng(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

#[inline]
pub fn random() -> f32 {
    random_rng(0.0, 1.0)
}

pub fn random_int_rng(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

#[inline]
pub fn reflect_ray(v: &Vector3, n: &Vector3) -> Vector3 {
    v - n * 2.0 * dot(v, n)
}

#[inline]
pub fn refract_ray(uv: &Vector3, n: &Vector3, etai_over_etat: f32) -> Vector3 {
    let cos_theta = f32::min(dot(&-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (n * cos_theta + uv);
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

    r_out_perp + r_out_parallel
}

pub fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

pub fn grad(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;

    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    let uu = if (h & 1) == 0 { u } else { -u };
    let vv = if (h & 2) == 0 { v } else { -v };

    uu + vv
}
