use rand::Rng;
use vector::Vector3;

pub mod camera;
pub mod interval;
pub mod material;
pub mod ray;
pub mod shape;
pub mod vector;
pub mod hittable;

pub fn dot(fst: &Vector3, snd: &Vector3) -> f32 {
    fst.dot(snd)
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0..=1.0)
}

pub fn random_rng(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}

pub fn reflect_ray(v: &Vector3, n: &Vector3) -> Vector3 {
    v - n * 2.0 * dot(v, n)
}
