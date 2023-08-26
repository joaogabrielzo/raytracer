use crate::{
    ray::Ray,
    shape::{HitRecord, Hittable, HittableList},
    vector::{Color, Point, Vector3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub image_height: u32,
    pub center: Point,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
}

impl Camera {
    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for v in 0..self.image_height {
            for u in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (u as f32 * self.pixel_delta_u)
                    + (v as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&ray, &world);

                pixel_color.write();
            }
        }
    }

    pub fn new(aspect_ratio: f32, image_width: u32) -> Camera {
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
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(ray, &(0.0, f32::MAX).into(), &mut rec) {
            return 0.5 * (rec.normal + Color::from_one(1.0));
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);

        // LERP -> (1 - a) * startValue + a * endValue
        (1.0 - a) * Color::from_one(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
