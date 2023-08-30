use rand::Rng;
use vector::Vector3;

pub mod camera;
pub mod hittable;
pub mod interval;
pub mod material;
pub mod ray;
pub mod shape;
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
