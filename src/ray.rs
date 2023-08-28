use crate::vector::{Point, Vector3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        return self.origin + t * self.direction;
    }
}
