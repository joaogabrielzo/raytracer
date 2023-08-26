use raytracer::{
    ray::Ray,
    shape::{Element, HitRecord, Hittable, HittableList, Sphere},
    vector::{Color, Point, Vector3},
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let f32_width = image_width as f32;
    let image_height = (f32_width / aspect_ratio) as i32;
    let f32_height = image_height as f32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / f32_width;
    let pixel_delta_v = viewport_v / f32_height;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut world = HittableList::default();
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
    )));

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for v in 0..image_height {
        for u in 0..image_width {
            let pixel_center =
                pixel00_loc + (u as f32 * pixel_delta_u) + (v as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);

            pixel_color.write();
        }
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
