use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    shape::{sphere::Sphere, Element},
    texture::Texture,
    vector::{Point, Vector3, Color},
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    let earth = image::open("assets/earthmap.jpg").unwrap();
    let earth_tx = Texture::Image(earth);
    let earth_surface = Surface::Diffuse { albedo: earth_tx };
    let globe = Element::Sphere(Sphere::new(Point::zero(), 2., earth_surface));

    world.add(globe);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let fov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vector3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = Color::new(0.7, 0.8, 1.);

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        fov,
        look_from,
        look_at,
        view_up,
        defocus_angle,
        focus_dist,
        background
    );

    camera.render(&world);

    Ok(())
}
