use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    shape::{Element, Quad},
    texture::Texture,
    vector::{Color, Point, Vector3},
};
use std::io;

fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    let red = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.65, 0.05, 0.05)),
    };
    let white = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.73, 0.73, 0.73)),
    };
    let green = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.12, 0.45, 0.15)),
    };
    let light = Surface::DiffuseLight(Texture::SolidColor(Color::from_one(15.)));

    world.add(Element::Quad(Quad::new(
        Point::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        green,
    )));
    world.add(Element::Quad(Quad::new(
        Point::new(0., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        red,
    )));
    world.add(Element::Quad(Quad::new(
        Point::new(343., 554., 332.),
        Vector3::new(-130., 0., 0.),
        Vector3::new(0., 0., -105.),
        light,
    )));
    world.add(Element::Quad(Quad::new(
        Point::new(0., 0., 0.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Element::Quad(Quad::new(
        Point::new(555., 555., 555.),
        Vector3::new(-555., 0., 0.),
        Vector3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Element::Quad(Quad::new(
        Point::new(0., 0., 555.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        white,
    )));

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 200;
    let max_depth = 50;

    let fov = 40.0;
    let look_from = Point::new(278.0, 278.0, -800.0);
    let look_at = Point::new(278.0, 278.0, 0.0);
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
