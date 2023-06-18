use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::clamp;
use raytracing::material::Lambertian;
use raytracing::material::Metal;
use raytracing::random_float;
use raytracing::sphere::Sphere;
use raytracing::vector::*;
use raytracing::HittableList;

fn main() {
    let aspect_ratio = 16. / 9.;
    let width = 400;
    let height = (width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Materials
    let ground_material = Rc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let center_material = Rc::new(Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });
    let left_material = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let right_material = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(Box::new(Sphere {
        center: Point::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: ground_material,
    }));
    world.add(Box::new(Sphere {
        center: Point::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: center_material,
    }));
    world.add(Box::new(Sphere {
        center: Point::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: left_material,
    }));
    world.add(Box::new(Sphere {
        center: Point::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: right_material,
    }));

    // Camera
    let camera = Camera::default();

    println!("P3");
    println!("{width} {height}");
    println!("255");

    for j in (0..height).rev() {
        for i in 0..width {
            let mut color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_float(0.0, 1.0)) / (width as f32 - 1.0);
                let v = (j as f32 + random_float(0.0, 1.0)) / (height as f32 - 1.0);

                let mut ray = camera.get_ray(u, v);
                color += ray.color(&world, max_depth);
            }

            write_color(color, samples_per_pixel as f32);
        }
    }
}

fn write_color(color: Color, samples_per_pixel: f32) {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    )
}
