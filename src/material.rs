use crate::ray::*;
use crate::vector::*;
use crate::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        scattered.origin = rec.p;
        scattered.direction = scatter_direction;

        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;

        true
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        mut attenuation: &mut Color,
        mut scattered: &mut Ray,
    ) -> bool {
        let normalized_in_direction = r_in.direction.normalize();

        let reflected = normalized_in_direction.reflect(&rec.normal);

        scattered.origin = rec.p;
        scattered.direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();

        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;

        scattered.direction.dot(&rec.normal) > 0.0
    }
}
