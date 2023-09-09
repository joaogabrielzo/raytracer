use itertools::Itertools;
use rand::Rng;
use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    noise::perlin::Perlin,
    shape::{a_box::a_box, Element, Quad, Sphere},
    texture::Texture,
    vector::{Color, Point, Vector3},
};
use std::io;

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::default();

    let ground = Surface::Diffuse {
        albedo: Vector3::new(0.48, 0.83, 0.53).into(),
    };

    let boxes_per_side = 20;
    for (j, i) in (0..boxes_per_side).cartesian_product(0..boxes_per_side) {
        let w = 100.0;
        let x0 = -1000.0 + i as f32 * w;
        let z0 = -1000.0 + j as f32 * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = rng.gen_range(1f32..101.);
        let z1 = z0 + w;

        world.add(Element::Box(a_box(
            &Point::new(x0, y0, z0),
            &Point::new(x1, y1, z1),
            ground.clone(),
        )));
    }

    let light = Surface::DiffuseLight(Vector3::new(7., 7., 7.).into());
    world.add(Element::Quad(Quad::new(
        Vector3::new(123., 554., 147.),
        Vector3::new(300., 0., 0.),
        Vector3::new(0., 0., 265.),
        light,
    )));

    let center1 = Vector3::new(400., 400., 200.);
    let center2 = center1 + Vector3::new(30., 0., 0.);
    let sphere_surface = Surface::Diffuse {
        albedo: Vector3::new(0.7, 0.3, 0.1).into(),
    };
    world.add(Element::Sphere(Sphere::new_moving(
        center1,
        center2,
        50.,
        sphere_surface,
    )));

    world.add(Element::Sphere(Sphere::new(
        Vector3::new(260., 150., 45.),
        50.,
        Surface::Refractive {
            idx_of_refraction: 1.5,
        },
    )));
    world.add(Element::Sphere(Sphere::new(
        Vector3::new(0., 150., 145.),
        50.,
        Surface::Reflective {
            albedo: Vector3::new(0.8, 0.8, 0.9),
            fuzz: 1.0,
        },
    )));

    let boundary = Element::Sphere(Sphere::new(
        Vector3::new(360., 150., 145.),
        70.,
        Surface::Refractive {
            idx_of_refraction: 1.5,
        },
    ));
    world.add(boundary);

    let earth = image::open("assets/earthmap.jpg").unwrap();
    let earth_tx = Texture::Image(earth);
    let earth_surface = Surface::Diffuse { albedo: earth_tx };
    world.add(Element::Sphere(Sphere::new(
        Vector3::new(400., 200., 400.),
        100.,
        earth_surface,
    )));

    let perlin = Perlin::default();
    let pertext = Texture::Perlin(perlin);
    world.add(Element::Sphere(Sphere::new(
        Vector3::new(220., 280., 300.),
        80.,
        Surface::Diffuse { albedo: pertext },
    )));

    let aspect_ratio = 1.0;
    let image_width = 800;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let fov = 40.0;
    let look_from = Point::new(478., 278., -600.);
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
