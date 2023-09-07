use crate::vector::{Point, Vector3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
