use crate::{
    hittable::{Hittable, HittableList},
    material::Material,
    random,
    ray::Ray,
    vector::{Color, Point, Vector3},
};

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub center: Point,
    pub pixel00_loc: Point,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        fov: f32,
        look_from: Point,
        look_at: Point,
        view_up: Vector3,
    ) -> Camera {
        let f32_width = image_width as f32;
        let image_height = (f32_width / aspect_ratio) as u32;
        let f32_height = image_height as f32;

        // Camera
        let look_direction = look_from - look_at;
        let focal_length = look_direction.length();
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;
        let center = look_from;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = look_direction.unit();
        let u = view_up.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / f32_width;
        let pixel_delta_v = viewport_v / f32_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (w * focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        (0..self.image_height).for_each(|v| {
            (0..self.image_width).for_each(|u| {
                let pixel_color: Color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let ray = self.get_ray(u, v);
                        Self::ray_color(&ray, world, self.max_depth)
                    })
                    .sum();

                pixel_color.write(self.samples_per_pixel as f32);
            })
        });
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
        if depth == 0 {
            return Color::black();
        }

        if let Some(rec) = world.hit(ray, &(0.001, f32::MAX).into()) {
            if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            };
            return Color::black();
        }

        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);

        // LERP -> (1 - a) * startValue + a * endValue
        (1.0 - a) * Color::white() + a * Color::new(0.5, 0.7, 1.0)
    }
}
