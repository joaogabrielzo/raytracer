use crate::{
    dot,
    hittable::HitRecord,
    random,
    ray::Ray,
    reflect_ray, refract_ray,
    vector::{Color, Vector3},
};

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone)]
pub enum Surface {
    Diffuse(Diffuse),
    Reflective(Metal),
    Refractive(Dielectric),
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
            Surface::Refractive(ref d) => d.scatter(ray_in, rec),
        }
    }
}

#[derive(Default, Clone)]
pub struct Diffuse {
    pub albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        Some((scattered, self.albedo))
    }
}

#[derive(Default, Clone)]
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
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vector3::random_unit_vector(),
            ray_in.time,
        );

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
pub struct Dielectric {
    pub ir: f32, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for Dielectric {
    #[allow(unused_assignments)]
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = f32::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut direction = Vector3::zero();

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
            direction = reflect_ray(&unit_direction, &rec.normal);
        } else {
            direction = refract_ray(&unit_direction, &rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction, ray_in.time);

        Some((scattered, Color::white()))
    }
}
