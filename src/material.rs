use crate::{
    dot,
    hittable::HitRecord,
    random,
    ray::Ray,
    reflect_ray, refract_ray,
    texture::Texture,
    vector::{Color, Point, Vector3},
};

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    #[allow(unused_variables)]
    fn emitted(&self, u: f32, v: f32, point: &Point) -> Color {
        Color::black()
    }
}

#[derive(Clone)]
pub enum Surface {
    Diffuse { albedo: Texture },
    Reflective { albedo: Color, fuzz: f32 },
    Refractive { idx_of_refraction: f32 },
    DiffuseLight(Texture),
}

impl Default for Surface {
    fn default() -> Self {
        Surface::Diffuse {
            albedo: Texture::SolidColor(Color::black()),
        }
    }
}

impl Material for Surface {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Surface::Diffuse { albedo } => {
                let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
                Some((scattered, albedo.color(rec.u, rec.v, &rec.p)))
            }
            Surface::Reflective { albedo, fuzz } => {
                let reflected = reflect_ray(&ray_in.direction, &rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + Vector3::random_unit_vector() * *fuzz,
                    ray_in.time,
                );

                if dot(&scattered.direction, &rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
            Surface::Refractive { idx_of_refraction } => {
                let refraction_ratio = if rec.front_face {
                    1.0 / idx_of_refraction
                } else {
                    *idx_of_refraction
                };

                let unit_direction = ray_in.direction.unit();
                let cos_theta = f32::min(dot(&-unit_direction, &rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction =
                    if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
                        reflect_ray(&unit_direction, &rec.normal)
                    } else {
                        refract_ray(&unit_direction, &rec.normal, refraction_ratio)
                    };

                let scattered = Ray::new(rec.p, direction, ray_in.time);

                Some((scattered, Color::white()))
            }
            Surface::DiffuseLight(_) => None,
        }
    }

    fn emitted(&self, u: f32, v: f32, point: &Point) -> Color {
        match self {
            Surface::DiffuseLight(emit) => emit.color(u, v, point),
            _ => Color::black(),
        }
    }
}

struct DiffuseLight {
    pub emit: Texture,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Point) -> Color {
        self.emit.color(u, v, p)
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
