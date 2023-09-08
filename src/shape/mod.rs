mod quad;
pub mod sphere;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub use self::quad::Quad;
pub use self::sphere::Sphere;

pub enum Element {
    Sphere(Sphere),
    Quad(Quad),
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match *self {
            Element::Sphere(ref s) => s.hit(ray, ray_t),
            Element::Quad(ref q) => q.hit(ray, ray_t),
        }
    }
}
