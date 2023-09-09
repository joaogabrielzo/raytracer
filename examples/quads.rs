use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    shape::{Element, Quad},
    texture::Texture,
    vector::{Color, Point, Vector3},
};
use std::io;

#[rustfmt::skip]
fn main() -> io::Result<()> {
    let mut world = HittableList::default();

    // Materials
    let left_red = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(1.0, 0.2, 0.2)),
    };
    let back_green = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.2, 1.0, 0.2)),
    };
    let right_blue = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.2, 0.2, 1.0)),
    };
    let upper_orange = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(1.0, 0.5, 0.0)),
    };
    let lower_teal = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.2, 0.8, 0.8)),
    };

     // Quads
    world.add(Element::Quad(Quad::new(Point::new(-3.,-2., 5.), Vector3::new(0., 0.,-4.), Vector3::new(0., 4., 0.), left_red)));
    world.add(Element::Quad(Quad::new(Point::new(-2.,-2., 0.), Vector3::new(4., 0., 0.), Vector3::new(0., 4., 0.), back_green)));
    world.add(Element::Quad(Quad::new(Point::new( 3.,-2., 1.), Vector3::new(0., 0., 4.), Vector3::new(0., 4., 0.), right_blue)));
    world.add(Element::Quad(Quad::new(Point::new(-2., 3., 1.), Vector3::new(4., 0., 0.), Vector3::new(0., 0., 4.), upper_orange)));
    world.add(Element::Quad(Quad::new(Point::new(-2.,-3., 5.), Vector3::new(4., 0., 0.), Vector3::new(0., 0.,-4.), lower_teal)));

    let aspect_ratio = 1.0;
    let image_width = 400;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let fov = 80.0;
    let look_from = Point::new(0.0, 0.0, 9.0);
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
