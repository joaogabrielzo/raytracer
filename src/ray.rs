use crate::HittableList;
use crate::lerp;
use crate::vector::*;
use crate::HitRecord;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }

    pub fn color(&self, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(self, 0.0, f32::MAX, &mut rec) {
            return 0.5 * (rec.normal + Color::one());
        }

        let normal_dir = self.direction.normalize();
        let t: f32 = 0.5 * (normal_dir.y + 1.0);

        return lerp(t, Color::one(), Color::new(0.5, 0.7, 1.0));
    }

    pub fn hit_sphere(&self, center: &Point, radius: f32) -> f32 {
        let oc = self.origin - center.clone();
        let a = self.direction.magnitude_squared(); // self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = /* oc.dot(&oc) */ oc.magnitude_squared()  - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }
}
