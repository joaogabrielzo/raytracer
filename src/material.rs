use crate::{
    ray::Ray,
    reflect_ray,
    shape::HitRecord,
    vector::{Color, Vector3},
};

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub enum MaterialType {
    Diffuse(Diffuse),
    Reflective(Metal),
}

impl Default for MaterialType {
    fn default() -> Self {
        MaterialType::Diffuse(Diffuse::default())
    }
}

impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            MaterialType::Diffuse(ref d) => d.scatter(ray_in, rec),
            MaterialType::Reflective(ref m) => m.scatter(ray_in, rec),
        }
    }
}

#[derive(Default)]
pub struct Diffuse {
    pub albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        return Some((scattered, self.albedo));
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect_ray(&ray_in.direction, &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        return Some((scattered, self.albedo));
    }
}
