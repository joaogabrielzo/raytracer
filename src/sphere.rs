use crate::{ray::Ray, vector::Point, HitRecord, Hittable};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center.clone();
        let a = ray.direction.magnitude_squared();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the neares root that lies in the acceptable range.
        let mut root = (-b - sqrtd) / (2.0 * a);
        if root < t_min || t_max < root {
            root = (-b + sqrtd) / (2.0 * a);
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
