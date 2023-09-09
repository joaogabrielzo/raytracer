use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    noise::perlin::Perlin,
    shape::{sphere::Sphere, Element, Quad},
    texture::Texture,
    vector::{Color, Point, Vector3},
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    let pertext = Texture::Turbulence(Perlin::default());

    world.add(Element::Sphere(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        Surface::Diffuse {
            albedo: pertext.clone(),
        },
    )));

    world.add(Element::Sphere(Sphere::new(
        Point::new(0., 2., 0.),
        2.,
        Surface::Diffuse { albedo: pertext },
    )));

    let diff_light = Surface::DiffuseLight(Texture::SolidColor(Color::from_one(4.)));

    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, 7.0, 0.0),
        2.,
        diff_light.clone(),
    )));

    world.add(Element::Quad(Quad::new(
        Point::new(3., 1., -2.),
        Vector3::new(2., 0., 0.),
        Vector3::new(0., 2., 0.),
        diff_light,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let fov = 20.0;
    let look_from = Point::new(26.0, 3.0, 6.0);
    let look_at = Point::new(0.0, 2.0, 0.0);
    let view_up = Vector3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let background = Color::black();

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
        background,
    );

    camera.render(&world);

    Ok(())
}
