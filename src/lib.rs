
use std::time::Duration;
use std::io::{Error as IoError};

#[macro_use]
extern crate log;

extern crate hidapi;
use hidapi::{HidApi, HidDevice, HidError};

extern crate image;
use image::ImageError;

pub mod images;
pub use crate::images::{Colour, ImageOptions};

pub mod info;
pub use info::*;

/// StreamDeck object
pub struct StreamDeck {
    kind: Kind,
    device: HidDevice
}

/// Helper object for filtering device connections
#[cfg(feature = "structopt" )]
#[derive(structopt::StructOpt)]
pub struct Filter {
    #[structopt(long, default_value="0fd9", parse(try_from_str=u16_parse_hex), env="USB_VID")]
    /// USB Device Vendor ID (VID) in hex
    pub vid: u16,

    #[structopt(long, default_value="0063", parse(try_from_str=u16_parse_hex), env="USB_PID")]
    /// USB Device Product ID (PID) in hex
    pub pid: u16,

    #[structopt(long, env="USB_SERIAL")]
    /// USB Device Serial
    pub serial: Option<String>,
}

fn u16_parse_hex(s: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(s, 16)
}

#[derive(Debug)]
pub enum Error {
    Hid(HidError),
    Io(IoError),
    Image(ImageError),

    InvalidImageSize,
    InvalidKeyIndex,
    UnrecognisedPID,
    NoData,
}

impl From<HidError> for Error {
    fn from(e: HidError) -> Self {
        Self::Hid(e)
    }
}

/// Device USB Product Identifiers (PIDs)
pub mod pids {
    pub const ORIGINAL:     u16 = 0x0060;
    pub const ORIGINAL_V2:  u16 = 0x006d;
    pub const MINI:         u16 = 0x0063;
    pub const XL:           u16 = 0x006c;
}

impl StreamDeck {
    /// Connect to a streamdeck device
    pub fn connect(vid: u16, pid: u16, serial: Option<String>) -> Result<StreamDeck, Error> {
        // Create new API
        let api = HidApi::new()?;

        // Match info based on PID
        let kind = match pid {
            pids::ORIGINAL => Kind::Original,
            pids::MINI => Kind::Mini,

            pids::ORIGINAL_V2 => unimplemented!(),
            pids::XL => unimplemented!(),
            
            _ => return Err(Error::UnrecognisedPID),
        };

        debug!("Device info: {:?}", kind);

        // Attempt to connect to device
        let device = match &serial {
            Some(s) => api.open_serial(vid, pid, s),
            None => api.open(vid, pid),
        }?;

        // Return streamdeck object
        Ok(StreamDeck{device, kind})
    }

    /// Fetch the connected device kind
    /// 
    /// This can be used to retrieve related device information such as
    /// images sizes and modes
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Fetch the device manufacturer string
    pub fn manufacturer(&mut self) -> Result<String, Error> {
        let s = self.device.get_manufacturer_string()?;
        Ok(s.unwrap())
    }

    /// Fetch the device product string
    pub fn product(&mut self) -> Result<String, Error> {
        let s = self.device.get_product_string()?;
        Ok(s.unwrap())
    }

    /// Fetch the device serial
    pub fn serial(&mut self) -> Result<String, Error> {
        let s = self.device.get_serial_number_string()?;
        Ok(s.unwrap())
    }

    /// Fetch the device firmware version
    pub fn version(&mut self) -> Result<String, Error> {
        let mut buff = [0u8; 17];
        buff[0] = 0x04;

        let _s = self.device.get_feature_report(&mut buff)?;

        Ok(std::str::from_utf8(&buff[5..]).unwrap().to_string())
    }

    /// Reset the connected device
    pub fn reset(&mut self) -> Result<(), Error> {
        let mut cmd = [0u8; 17];
        
        cmd[..2].copy_from_slice(&[0x0b, 0x63]);

        self.device.send_feature_report(&cmd)?;

        Ok(())
    }

    /// Set the device display brightness (in percent)
    pub fn set_brightness(&mut self, brightness: u8) -> Result<(), Error> {
        let mut cmd = [0u8; 17];

        let brightness = brightness.min(100);

        cmd[..6].copy_from_slice(&[0x05, 0x55, 0xaa, 0xd1, 0x01, brightness]);

        self.device.send_feature_report(&cmd)?;

        Ok(())
    }

    /// Set blocking mode
    /// 
    /// See: `read_buttons` for discussion of this functionality
    pub fn set_blocking(&mut self, blocking: bool) -> Result<(), Error> {
        self.device.set_blocking_mode(blocking)?;

        Ok(())
    }

    /// Fetch button states
    /// 
    /// In blocking mode this will wait until a report packet has been received
    /// (or the specified timeout has elapsed). In non-blocking mode this will return
    /// immediately with a zero vector if no data is available
    pub fn read_buttons(&mut self, timeout: Option<Duration>) -> Result<Vec<u8>, Error> {
        let mut cmd = [0u8; 17];

        match timeout {
            Some(t) => self.device.read_timeout(&mut cmd, t.as_millis() as i32)?,
            None => self.device.read(&mut cmd)?,
        };

        if cmd[0] == 0 {
            return Err(Error::NoData)
        }

        Ok((&cmd[1..]).to_vec())
    }

    /// Fetch image size for the connected device
    pub fn image_size(&self) -> (usize, usize) {
        self.kind.image_size()
    }

    /// Set a button to the provided RGB colour
    pub fn set_button_rgb(&mut self, key: u8, colour: &Colour) -> Result<(), Error> {
        let mut image = vec![0u8; self.kind.image_size_bytes() ];

        for i in 0..image.len() {
            match i % 3 {
                0 => image[i] = colour.b,
                1 => image[i] = colour.g,
                2 => image[i] = colour.r,
                _ => unreachable!(),
            };
        }

        self.set_button_image(key, &image)?;

        Ok(())
    }

    /// Set a button to the provided image
    /// 
    /// Images in BGR format, `IMAGE_X` by `IMAGE_Y` at 3 bytes per pixel
    pub fn set_button_image(&mut self, key: u8, image: &[u8]) -> Result<(), Error> {
        match self.kind.image_mode() {
            ImageMode::Bmp => self.set_button_image_bmp(key, image),
            ImageMode::Jpeg => unimplemented!(),
        }
    }

    pub fn set_button_file(&mut self, key: u8, image: &str, opts: &ImageOptions) -> Result<(), Error> {
        let (x, y) = self.kind.image_size();
        let rotate = self.kind.image_rotation();

        let image = images::load_image(image, x, y, rotate, opts)?;

        self.set_button_image(key, &image)?;

        Ok(())
    }

    /// Internal function to set images for bitmap based devices
    fn set_button_image_bmp(&mut self, key: u8, image: &[u8]) -> Result<(), Error> {
        let mut buff = vec![0u8; self.kind.image_report_len() ];

        // Check image dimensions
        if image.len() != self.kind.image_size_bytes() {
            return Err(Error::InvalidImageSize)
        }

        // TODO: check / limit key value
        if key >= self.kind.keys() {
            return Err(Error::InvalidKeyIndex)
        }

        let mut sequence = 0;
        let mut offset = 0;
        
        while offset < image.len() {
            
            let mut index = 0;

            let overhead = match sequence {
                0 => 16 + 54,
                _ => 16,
            };

            // Calculate chunk size
            let max_chunk_size = self.kind.image_report_len() - overhead;
            let chunk_size = (image.len() - offset).min(max_chunk_size);

            trace!("sequence: {}, offset: {}, chunk_size: {}, buff_size: {}", sequence, offset, chunk_size, self.kind.image_report_len());

            // Build header
            let next = match chunk_size == (image.len() - offset) {
                true => 1,
                false => 0,
            };

            buff[..6].copy_from_slice(&[0x02, 0x01, sequence, 0x00, next, key + 1]);
            index += 16;

            // Add extra image header info to first message
            if sequence == 0 {
                let base = self.kind.image_base();
                buff[index..index+base.len()].copy_from_slice(&base);
                index += base.len();
            }

            // Copy image chunk
            buff[index..index+chunk_size].copy_from_slice(&image[offset..offset+chunk_size]);
            offset += chunk_size;
            index += chunk_size;

            // Zero out remaining message data
            for i in &mut buff[index..] {
                *i = 0;
            }

            trace!("Writing chunk");
            trace!("Header: {:x?}", &buff[..16]);
            trace!("Buffer: {:x?}", &buff[..]);

            self.device.write(&buff)?;

            // Increase sequence counter
            sequence += 1;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
