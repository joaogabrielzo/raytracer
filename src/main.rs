use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Diffuse, Metal, Surface},
    shape::{Element, Sphere},
    vector::{Color, Point},
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let material_ground = Surface::Diffuse(Diffuse::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Surface::Diffuse(Diffuse::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Surface::Refractive(Dielectric::new(1.5));
    let material_right = Surface::Reflective({
        let albedo = Color::new(0.8, 0.6, 0.2);
        Metal { albedo, fuzz: 0.0 }
    });

    let mut world = HittableList::default();
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    camera.render(&world);
}
