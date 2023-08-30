use crate::{
    interval::Interval,
    material::Surface,
    ray::Ray,
    shape::Element,
    vector::{Point, Vector3},
};

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector3,
    pub t: f32,
    pub material: &'a Surface,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point, normal: Vector3, t: f32, material: &'a Surface, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            material,
            front_face,
        }
    }

    /// Sets the hit record normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = -(*outward_normal)
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Element>,
}

impl HittableList {
    pub fn new(objects: Vec<Element>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, el: Element) {
        self.objects.push(el)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        self.objects.iter().for_each(|e| {
            if let Some(hit) = e.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        });

        rec
    }
}
