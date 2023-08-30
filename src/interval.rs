pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.min(1.0).max(0.0)
    }
}

impl From<(f32, f32)> for Interval {
    fn from(val: (f32, f32)) -> Self {
        Interval {
            min: val.0,
            max: val.1,
        }
    }
}
