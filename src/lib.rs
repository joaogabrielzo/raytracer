use vector::Vector3;

pub mod ray;
pub mod vector;
pub mod shape;
pub mod interval;
pub mod camera;

pub fn dot(fst: &Vector3, snd: &Vector3) -> f32 {
    fst.dot(snd)
}
