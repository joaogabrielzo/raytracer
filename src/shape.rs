use crate::{
    dot,
    ray::Ray,
    vector::{Point, Vector3}, interval::Interval,
};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

pub enum Element {
    Sphere(Sphere),
}

#[derive(Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
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
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        match *self {
            Element::Sphere(ref s) => s.hit(ray, ray_t, rec),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction); //a vector dotted with itself is equal to the squared length of that vector.
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) // Quadratic formula
                            / (2.0 * a);
        if root <= ray_t.min || root >= ray_t.max {
            root = (-b - sqrtd) / (2.0 * a);
            if root <= ray_t.min || root >= ray_t.max {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return true;
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
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        self.objects.iter().for_each(|e| {
            if e.hit(ray, ray_t, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        });

        return hit_anything;
    }
}
