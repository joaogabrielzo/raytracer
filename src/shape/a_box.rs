use crate::{
    hittable::HittableList,
    material::Surface,
    vector::{Point, Vector3},
};

use super::{Element, Quad};

pub fn a_box(a: &Point, b: &Point, material: Surface) -> HittableList {
    let mut sides = HittableList::default();

    let min = Point::new(f32::min(a.x, b.x), f32::min(a.y, b.y), f32::min(a.z, b.z));
    let max = Point::new(f32::max(a.x, b.x), f32::max(a.y, b.y), f32::max(a.z, b.z));

    let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vector3::new(0.0, max.y - min.y, 0.0);
    let dz = Vector3::new(0.0, 0.0, max.z - min.z);

    // front
    sides.add(Element::Quad(Quad::new(
        Point::new(min.x, min.y, max.z),
        dx,
        dy,
        material.clone(),
    )));
    // right
    sides.add(Element::Quad(Quad::new(
        Point::new(max.x, min.y, max.z),
        -dz,
        dy,
        material.clone(),
    )));
    // back
    sides.add(Element::Quad(Quad::new(
        Point::new(max.x, min.y, min.z),
        -dx,
        dy,
        material.clone(),
    )));
    // left
    sides.add(Element::Quad(Quad::new(
        Point::new(min.x, min.y, min.z),
        dz,
        dy,
        material.clone(),
    )));
    // top
    sides.add(Element::Quad(Quad::new(
        Point::new(min.x, max.y, max.z),
        dx,
        -dz,
        material.clone(),
    )));
    // bottom
    sides.add(Element::Quad(Quad::new(
        Point::new(min.x, min.y, min.z),
        dx,
        dz,
        material,
    )));

    sides
}
