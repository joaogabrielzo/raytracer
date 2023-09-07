use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    noise::perlin::Perlin,
    shape::{Element, Sphere},
    texture::Texture,
    vector::{Point, Vector3},
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    world.add(Element::Sphere(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        Surface::Diffuse {
            albedo: Texture::Perlin(Perlin::new()),
        },
    )));

    world.add(Element::Sphere(Sphere::new(
        Point::new(0., 2., 0.),
        2.,
        Surface::Diffuse {
            // albedo: Texture::Perlin(Perlin::new(1)),
            albedo: Texture::Perlin(raytracer::noise::perlin::Perlin::new()),
        },
    )));

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
    );

    camera.render(&world);

    Ok(())
}
