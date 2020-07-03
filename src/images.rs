use std::str::FromStr;

use image::io::Reader;
use image::jpeg::JPEGEncoder;
use image::DynamicImage;
use image::{imageops::FilterType, ColorType, Pixel, Rgba};

use crate::info::{ColourOrder, Mirroring, Rotation};
use crate::Error;

/// Simple Colour object for re-writing backgrounds etc.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "structopt", derive(structopt::StructOpt))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Colour {
    #[cfg_attr(feature = "structopt", structopt(long))]
    pub r: u8,

    #[cfg_attr(feature = "structopt", structopt(long))]
    pub g: u8,

    #[cfg_attr(feature = "structopt", structopt(long))]
    pub b: u8,
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 && s.len() != 8 {
            return Err(format!("Expected colour in the hex form: RRGGBB"));
        }

        let r =
            u8::from_str_radix(&s[0..2], 16).map_err(|e| format!("int parsing error: {}", e))?;
        let g =
            u8::from_str_radix(&s[2..4], 16).map_err(|e| format!("int parsing error: {}", e))?;
        let b =
            u8::from_str_radix(&s[4..6], 16).map_err(|e| format!("int parsing error: {}", e))?;

        Ok(Self { r, g, b })
    }
}

/// Options for image loading and editing
#[derive(Debug)]
#[cfg_attr(feature = "structopt", derive(structopt::StructOpt))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ImageOptions {
    #[cfg_attr(feature = "structopt", structopt(long = "bg"))]
    /// Background colour
    background: Option<Colour>,

    #[cfg_attr(feature = "structopt", structopt(long))]
    /// Invert colours
    invert: bool,
}

impl ImageOptions {
    pub fn new(background: Option<Colour>, invert: bool) -> Self {
        ImageOptions { background, invert }
    }
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {
            background: None,
            invert: false,
        }
    }
}

pub(crate) fn apply_transform(
    image: DynamicImage,
    rotation: Rotation,
    mirroring: Mirroring,
) -> DynamicImage {
    let image = match rotation {
        Rotation::Rot0 => image,
        Rotation::Rot90 => image.rotate90(),
        Rotation::Rot180 => image.rotate180(),
        Rotation::Rot270 => image.rotate270(),
    };
    let image = match mirroring {
        Mirroring::None => image,
        Mirroring::X => image.flipv(),
        Mirroring::Y => image.fliph(),
        Mirroring::Both => image.flipv().fliph(),
    };
    image
}

/// Load an image from a file, resize to defined x and y, and apply the provided options
pub(crate) fn load_image(
    path: &str,
    x: usize,
    y: usize,
    rotate: Rotation,
    mirror: Mirroring,
    opts: &ImageOptions,
    colour_order: ColourOrder,
) -> Result<Vec<u8>, Error> {
    // Open image reader
    let reader = match Reader::open(path) {
        Ok(v) => v,
        Err(e) => {
            error!("error loading file '{}': {:?}", path, e);
            return Err(Error::Io(e));
        }
    };

    // Load image
    let mut image = reader.decode().map_err(Error::Image)?;

    // Apply background filter / replace
    // This must be done before transparency is removed
    if let Some(c) = &opts.background {
        let rgba = image.as_mut_rgba8().unwrap();

        let mut r = Rgba([c.r, c.g, c.b, 0]);
        if opts.invert {
            r.invert();
        }

        for p in rgba.pixels_mut() {
            r.0[3] = 255 - p.0[3];

            p.blend(&r);
        }
    }

    // Resize image
    let mut image = image.resize(x as u32, y as u32, FilterType::Gaussian);

    // Apply the requested mirroring transformation
    image = apply_transform(image, rotate, mirror);

    // Invert image if requir
    if opts.invert {
        image.invert();
    }

    // Convert to vector with correct encoding
    let v = match colour_order {
        ColourOrder::BGR => image.to_bgr().into_vec(),
        ColourOrder::RGB => image.to_rgb().into_vec(),
    };

    if v.len() != x * y * 3 {
        return Err(Error::InvalidImageSize);
    }

    Ok(v)
}

/// Encodes a BGR bitmap into a JPEG image for outputting to a V2 device
pub(crate) fn encode_jpeg(image: &[u8], width: usize, height: usize) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    let mut encoder = JPEGEncoder::new_with_quality(&mut buf, 100);
    encoder.encode(image, width as u32, height as u32, ColorType::Rgb8)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_images() {
        let _image = load_image(
            "./icons/power.png",
            72,
            72,
            Rotation::Rot180,
            Mirroring::Both,
            &ImageOptions::default(),
            ColourOrder::BGR,
        )
        .expect("error loading image");
    }
}
