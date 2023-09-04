use std::f32::consts::PI;

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
    pub is_moving: bool,
    pub center_vec: Point,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Surface) -> Self {
        Self {
            center,
            radius,
            radius_squared: radius * radius,
            material,
            is_moving: false,
            center_vec: center,
        }
    }

    pub fn new_moving(center1: Point, center2: Point, radius: f32, material: Surface) -> Self {
        Self {
            center: center1,
            radius,
            radius_squared: radius * radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
        }
    }

    pub fn center(&self, time: f32) -> Point {
        self.center + self.center_vec * time
    }

    fn get_sphere_uv(&self, p: Vector3) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.center(ray.time)
        } else {
            self.center
        };
        let oc = ray.origin - center;
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

        let point = ray.at(root);
        let outward_normal = (point - center) / self.radius;

        let (u, v) = self.get_sphere_uv(outward_normal);

        let mut rec = HitRecord::new(point, Vector3::zero(), root, &self.material, false, u, v);
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
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
