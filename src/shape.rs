use crate::{
    dot,
    interval::Interval,
    material::MaterialType,
    ray::Ray,
    vector::{Point, Vector3},
};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: MaterialType,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: MaterialType) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

pub enum Element {
    Sphere(Sphere),
}

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector3,
    pub t: f32,
    pub material: &'a MaterialType,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point,
        normal: Vector3,
        t: f32,
        material: &'a MaterialType,
        front_face: bool,
    ) -> Self {
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

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match *self {
            Element::Sphere(ref s) => s.hit(ray, ray_t),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction); //a vector dotted with itself is equal to the squared length of that vector.
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) // Quadratic formula
                            / (2.0 * a);
        if root <= ray_t.min || root >= ray_t.max {
            root = (-b - sqrtd) / (2.0 * a);
            if root <= ray_t.min || root >= ray_t.max {
                return None;
            }
        }

        let mut rec = HitRecord::new(ray.at(root), Vector3::zero(), root, &self.material, false);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return Some(rec);
    }
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
            if let Some(hit) = e.hit(ray, ray_t) {
                closest_so_far = hit.t;
                rec = Some(hit);
            }
        });

        return rec;
    }
}
