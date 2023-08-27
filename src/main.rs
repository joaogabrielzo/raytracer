use raytracer::{
    camera::Camera,
    shape::{Element, HittableList, Sphere},
    vector::Point,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let mut world = HittableList::default();
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
    )));
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
    )));

    camera.render(&world);
}
