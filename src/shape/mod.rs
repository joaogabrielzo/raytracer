pub mod a_box;
mod quad;
pub mod sphere;

use crate::{
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    vector::Vector3,
};

pub use self::quad::Quad;
pub use self::sphere::Sphere;

pub enum Element {
    Sphere(Sphere),
    Quad(Quad),
    Box(HittableList),
    Translate {
        offset: Vector3,
        object: Box<Element>,
    },
    RotateY {
        sin_theta: f32,
        cos_theta: f32,
        object: Box<Element>,
    },
}

impl Element {
    pub fn new_rotate_y(angle: f32, object: Element) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        Self::RotateY {
            sin_theta,
            cos_theta,
            object: Box::new(object),
        }
    }
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Element::Sphere(ref s) => s.hit(ray, ray_t),
            Element::Quad(ref q) => q.hit(ray, ray_t),
            Element::Box(ref b) => b.hit(ray, ray_t),
            Element::Translate { offset, object } => {
                // Move the ray backwards by the offset
                let offset_ray = Ray {
                    origin: ray.origin - offset,
                    direction: ray.direction,
                    time: ray.time,
                };
                // Determine where (if any) an intersection occurs along the offset ray
                let Some(mut hit_record) =
                    object.hit(&offset_ray, ray_t)
                else {
                    return None;
                };
                // Move the intersection point forwards by the offset
                hit_record.p += *offset;
                Some(hit_record)
            }
            Element::RotateY {
                sin_theta,
                cos_theta,
                object,
            } => {
                // Change the ray from world space to object space
                let mut origin = ray.origin.clone();
                let mut direction = ray.direction.clone();

                origin.x = cos_theta * ray.origin.x - sin_theta * ray.origin.z;
                origin.z = sin_theta * ray.origin.x + cos_theta * ray.origin.z;

                direction.x = cos_theta * ray.direction.x - sin_theta * ray.direction.z;
                direction.z = sin_theta * ray.direction.x + cos_theta * ray.direction.z;

                let rotated_r = Ray {
                    origin,
                    direction,
                    time: ray.time,
                };

                // Determine where (if any) an intersection occurs in object space
                let Some(mut hit_record) =
                    object.hit(&rotated_r, ray_t)
                else {
                    return None;
                };

                // Change the intersection point from object space to world space
                let mut p = hit_record.p;
                p.x = cos_theta * hit_record.p.x + sin_theta * hit_record.p.z;
                p.z = -sin_theta * hit_record.p.x + cos_theta * hit_record.p.z;

                // Change the normal from object space to world space
                let mut normal = hit_record.normal;
                normal.x = cos_theta * hit_record.normal.x + sin_theta * hit_record.normal.z;
                normal.z = -sin_theta * hit_record.normal.x + cos_theta * hit_record.normal.z;

                hit_record.p = p;
                hit_record.normal = normal;

                Some(hit_record)
            }
        }
    }
}
