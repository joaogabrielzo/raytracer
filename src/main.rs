use raytracer::{
    camera::Camera,
    hittable::HittableList,
    material::Surface,
    random, random_rng,
    shape::{sphere::Sphere, Element},
    texture::Texture,
    vector::{Color, Point, Vector3},
};

#[allow(unused_assignments)]
fn main() {
    let checker = Texture::Checkered {
        even: Color::new(0.2, 0.3, 0.1),
        odd: Color::from_one(0.9),
        scale: 0.32,
    };
    let material_ground = Surface::Diffuse { albedo: checker };
    let mut world = HittableList::default();
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let choose_material = random();
            let center = Point::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere_material = Surface::default();

                if choose_material < 0.8 {
                    let albedo = Texture::SolidColor(Color::random() * Color::random());
                    sphere_material = Surface::Diffuse { albedo };
                    let center2 = center + Vector3::new(0.0, random_rng(0.0, 0.5), 0.0);
                    world.add(Element::Sphere(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    let albedo = Color::random_rng(0.5, 1.0);
                    let fuzz = random_rng(0.0, 0.5);
                    sphere_material = Surface::Reflective { albedo, fuzz };
                    world.add(Element::Sphere(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Surface::Refractive {
                        idx_of_refraction: 1.5,
                    };
                    world.add(Element::Sphere(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        })
    });

    let material_one = Surface::Refractive {
        idx_of_refraction: 1.5,
    };
    world.add(Element::Sphere(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material_one,
    )));

    let material_two = Surface::Diffuse {
        albedo: Texture::SolidColor(Color::new(0.4, 0.2, 0.1)),
    };
    world.add(Element::Sphere(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_two,
    )));

    let material_three = Surface::Reflective {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Element::Sphere(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_three,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let fov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let view_up = Vector3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
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
        background,
    );

    camera.render(&world);
}
