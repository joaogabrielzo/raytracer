use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use crate::random_float;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();

        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(&self, rhs: &Vec3) -> Vec3 {
        let reflect_diretion = 2.0 * self.dot(rhs) * rhs;

        *self - reflect_diretion
    }

    pub fn refract(&self, rhs: &Vec3, etai_over_etat: f32) -> Vec3 {
        let negative_self = -1.0 * self;
        let cos_theta = negative_self.dot(rhs).min(1.0);

        let r_out_perpendicular = etai_over_etat * (self + cos_theta * rhs);
        let r_out_parallel =
            ((1.0 - r_out_perpendicular.magnitude_squared()).abs().sqrt() * -1.0) * rhs;

        r_out_perpendicular + r_out_parallel
    }

    pub fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_float(-1.0, 1.0),
            y: random_float(-1.0, 1.0),
            z: random_float(-1.0, 1.0),
        }
    }

    pub fn random_between(min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: random_float(min, max),
            y: random_float(min, max),
            z: random_float(min, max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut vec = Vec3::zero();

        let mut inside_sphere = false;

        while !inside_sphere {
            let p = Vec3::random();

            if p.magnitude_squared() < 1.0 {
                inside_sphere = true;
                vec = p;
            } else {
                continue;
            }
        }

        vec
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();

        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            in_unit_sphere * -1.0
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut vec = Vec3::default();
        let mut inside_disk = true;

        while inside_disk {
            let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);

            if p.magnitude_squared() < 1.0 {
                inside_disk = false;
                vec = p;
            } else {
                continue;
            }
        }

        vec
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(val: (f32, f32, f32)) -> Self {
        Vec3 {
            x: val.0,
            y: val.1,
            z: val.2,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f32> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
