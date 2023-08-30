use crate::{
    dot,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Surface,
    ray::Ray,
    vector::{Point, Vector3},
};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub radius_squared: f32,
    pub material: Surface,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Surface) -> Self {
        Self {
            center,
            radius,
            radius_squared: radius * radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction); //a vector dotted with itself is equal to the squared length of that vector.
        let b = 2.0 * dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius_squared;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-b - sqrtd) // Quadratic formula
                            / (2.0 * a);
        if root <= ray_t.min || root >= ray_t.max {
            root = (-b + sqrtd) / (2.0 * a);
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

pub enum Element {
    Sphere(Sphere),
}

impl Hittable for Element {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match *self {
            Element::Sphere(ref s) => s.hit(ray, ray_t),
        }
    }
}
