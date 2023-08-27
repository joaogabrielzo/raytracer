use crate::{
    random,
    ray::Ray,
    shape::{HitRecord, Hittable, HittableList},
    vector::{Color, Point, Vector3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub center: Point,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Camera {
        let f32_width = image_width as f32;
        let image_height = (f32_width / aspect_ratio) as u32;
        let f32_height = image_height as f32;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let center = Point::zero();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / f32_width;
        let pixel_delta_v = viewport_v / f32_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for v in 0..self.image_height {
            for u in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(u, v);

                    pixel_color += Self::ray_color(&ray, &world, self.max_depth);
                }

                pixel_color.write(self.samples_per_pixel as f32);
            }
        }
    }

    fn get_ray(&self, u: u32, v: u32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (u as f32 * self.pixel_delta_u) + (v as f32 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let px = -0.5 + random();
        let py = -0.5 + random();

        (self.pixel_delta_u * px) + (self.pixel_delta_v * py)
    }

    fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
        let mut rec = HitRecord::default();

        if depth == 0 {
            return Color::zero();
        }

        if world.hit(ray, &(0.001, f32::MAX).into(), &mut rec) {
            let direction = rec.normal + Vector3::random_unit_vector();
            return 0.5 * Self::ray_color(&Ray::new(rec.p, direction), world, depth - 1);
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);

        // LERP -> (1 - a) * startValue + a * endValue
        (1.0 - a) * Color::from_one(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
