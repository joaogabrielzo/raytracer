use std::{io, path::Path};

use image::{DynamicImage, GenericImageView};

use crate::vector::Vector3;

#[derive(Clone)]
pub enum Texture {
    SolidColor(Vector3),
    Checkered {
        even: Vector3,
        odd: Vector3,
        scale: f32,
    },
    Image(DynamicImage),
}
impl Texture {
    pub fn load_image<P>(path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        use image::io::Reader as ImageReader;

        let img = ImageReader::open(path)?.decode().unwrap();

        Ok(Self::Image(img))
    }
    pub fn color(&self, u: f32, v: f32, point: Vector3) -> Vector3 {
        match self {
            Texture::SolidColor(color) => *color,
            Texture::Checkered { even, odd, scale } => {
                let x_integer = (scale.recip() * point.x).floor() as i32;
                let y_integer = (scale.recip() * point.y).floor() as i32;
                let z_integer = (scale.recip() * point.z).floor() as i32;

                let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

                if is_even {
                    *even
                } else {
                    *odd
                }
            }
            Texture::Image(image) => {
                // If we have no texture data, then return solid cyan as a debugging aid.
                if image.height() <= 0 {
                    return Vector3::new(0., 1., 1.);
                }
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

                let i: u32 = (u * image.width() as f32) as u32;
                let j: u32 = (v * image.height() as f32) as u32;

                let pixel = image.get_pixel(i, j);

                let color_scale = 1.0 / 255.0;
                return Vector3::new(
                    color_scale * pixel[0] as f32,
                    color_scale * pixel[1] as f32,
                    color_scale * pixel[2] as f32,
                );
            }
        }
    }
}

impl From<Vector3> for Texture {
    fn from(value: Vector3) -> Self {
        Self::SolidColor(value)
    }
}
