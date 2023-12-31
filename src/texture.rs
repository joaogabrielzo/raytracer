use std::{io, path::Path};

use image::{DynamicImage, GenericImageView};

use crate::{
    noise::perlin::Perlin,
    vector::{Color, Point},
};

#[derive(Clone)]
pub enum Texture {
    SolidColor(Color),
    Checkered { even: Color, odd: Color, scale: f32 },
    Image(DynamicImage),
    Perlin(Perlin),
    Turbulence(Perlin),
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

    pub fn color(&self, u: f32, v: f32, point: &Point) -> Color {
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
                if image.height() == 0 {
                    return Color::new(0., 1., 1.);
                }
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

                let i: u32 = (u * image.width() as f32) as u32;
                let j: u32 = (v * image.height() as f32) as u32;

                let pixel = image.get_pixel(i, j);

                let color_scale = 1.0 / 255.0;
                Color::new(
                    color_scale * pixel[0] as f32,
                    color_scale * pixel[1] as f32,
                    color_scale * pixel[2] as f32,
                )
            }
            Texture::Perlin(perlin) => {
                // let noise = perlin.get(((point * 5.) + 1.0 / 2.0).to_array());

                let noise = perlin.improved_noise(point);

                Color::white() * noise
            }
            Texture::Turbulence(perlin) => {
                let noise = perlin.turbulence(point, 7);

                Color::white() * noise
            }
        }
    }
}

impl From<Color> for Texture {
    fn from(value: Color) -> Self {
        Self::SolidColor(value)
    }
}
