use crate::{
    dot,
    hittable::HitRecord,
    ray::Ray,
    reflect_ray,
    vector::{Color, Vector3},
};

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub enum Surface {
    Diffuse(Diffuse),
    Reflective(Metal),
}

impl Default for Surface {
    fn default() -> Self {
        Surface::Diffuse(Diffuse::default())
    }
}

impl Material for Surface {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Surface::Diffuse(ref d) => d.scatter(ray_in, rec),
            Surface::Reflective(ref m) => m.scatter(ray_in, rec),
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
        Some((scattered, self.albedo))
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = fuzz.min(1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect_ray(&ray_in.direction, &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vector3::random_unit_vector());

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
