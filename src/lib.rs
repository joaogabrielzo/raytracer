pub mod camera;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vector;

use std::rc::Rc;

use material::{Lambertian, Material};
use ray::*;
use vector::*;

pub fn lerp(t: f32, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    return x;
}

pub fn random_float(min: f32, max: f32) -> f32 {
    let random: f32 = rand::random();

    min + (max - min) * random
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub material: Rc<dyn Material>,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
            t: Default::default(),
            material: Rc::new(Lambertian {
                albedo: Color::new(0.4, 0.4, 0.9),
            }),
            front_face: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            outward_normal.clone() * -1.0
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, mut rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord::default();

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        rec.front_face = temp_rec.front_face;
        rec.normal = temp_rec.normal;
        rec.p = temp_rec.p;
        rec.t = temp_rec.t;
        rec.material = temp_rec.material;

        hit_anything
    }
}
