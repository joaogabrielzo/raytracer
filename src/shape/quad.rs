use crate::{
    dot,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Surface,
    ray::Ray,
    vector::{Point, Vector3},
};

pub struct Quad {
    pub q: Point,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
    pub material: Surface,
    pub normal: Vector3,
    pub d: f32,
}

impl Quad {
    pub fn new(q: Point, u: Vector3, v: Vector3, material: Surface) -> Self {
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / dot(&n, &n);

        Self {
            q,
            u,
            v,
            w,
            material,
            normal,
            d,
        }
    }

    fn is_interior(a: f32, b: f32) -> (f32, f32) {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise returns hit record UV coordinates and return true.

        if a < 0. || a > 1. || b < 0. || b > 1. {
            return (0., 0.);
        }

        return (a, b);
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        let n1e_8 = 1.0 * 10.0f32.powf(-8.0); // 1e-8
        if denom.abs() < n1e_8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(&ray.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }
        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpoint = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpoint.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpoint));

        let (u, v) = Self::is_interior(alpha, beta);

        if u == 0. && v == 0. {
            return None;
        }

        let mut rec = HitRecord::new(
            intersection,
            Vector3::zero(),
            t,
            &self.material,
            false,
            u,
            v,
        );
        rec.set_face_normal(ray, &self.normal);

        Some(rec)
    }
}
