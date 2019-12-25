use std::str::FromStr;

use image::{FilterType, Rgba, Pixel};
use image::io::Reader;

use crate::Error;

/// Simple Colour object for re-writing backgrounds etc.
#[derive(Debug)]
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

        let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| format!("int parsing error: {}", e))?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| format!("int parsing error: {}", e))?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| format!("int parsing error: {}", e))?;

        Ok(Self{r, g, b})
    }
}

/// Options for image loading and editing
#[derive(Debug)]
#[cfg_attr(feature = "structopt", derive(structopt::StructOpt))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))] 
pub struct ImageOptions {  
    #[cfg_attr(feature = "structopt", structopt(long="bg"))]
    /// Background colour
    background: Option<Colour>,

    #[cfg_attr(feature = "structopt", structopt(long))]
    /// Invert colours
    invert: bool,
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self{
            background: None,
            invert: false,
        }
    }
}

/// Load an image from a file, resize to defined x and y, and apply the provided options
pub fn load_image(path: &str, x: usize, y: usize, rotate: bool, opts: &ImageOptions) -> Result<Vec<u8>, Error> {

    // Open image reader
    let reader = match Reader::open(path) {
        Ok(v) => v,
        Err(e) => {
            error!("error loading file '{}': {:?}", path, e);
            return Err(Error::Io(e))
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
            r.0[3] = 255-p.0[3];

            p.blend(&r);
        }
    }

    // Resize image
    let mut image = image.resize(x as u32, y as u32, FilterType::Gaussian);

    // Rotate image if requir
    if rotate {
        image = image.rotate270();
    }

    // Invert image if requir
    if opts.invert {
        image.invert();
    }

    // Convert to bgr8 encoding
    let bgr = image.to_bgr();

    // Fetch vector
    let v = bgr.into_vec();

    assert_eq!(v.len(), x * y * 3);

    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_images() {
        let _image = load_image("./icons/power.png", 72, 72, true, &ImageOptions::default())
            .expect("error loading image");
    }
}