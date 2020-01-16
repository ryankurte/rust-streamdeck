
/// Stream Deck Device Kinds
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Kind {
    Original,
    OriginalV2,
    Mini,
    Xl,
}

/// Stream Deck Image Modes
#[derive(Debug, Clone, PartialEq)]
pub enum ImageMode {
    Bmp,
    Jpeg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    Rot0,
    Rot90,
}

const ORIGINAL_IMAGE_REPORT_LEN: usize = 8191;
const MINI_IMAGE_REPORT_LEN: usize = 1024;

impl Kind {
    pub fn keys(&self) -> u8 {
        match self {
            Kind::Original | Kind::OriginalV2 => 16,
            Kind::Mini => 8,
            Kind::Xl => 32,
        }
    }

    pub fn image_mode(&self) -> ImageMode {
        match self {
            Kind::Original | Kind::Mini => ImageMode::Bmp,
            Kind::OriginalV2 | Kind::Xl => ImageMode::Jpeg,
        }
    }

    pub fn image_size(&self) -> (usize, usize) {
        match self {
            Kind::Original | Kind::OriginalV2 => (72, 72),
            Kind::Mini => (80, 80),
            Kind::Xl => (96, 96),
        }
    }

    pub fn image_rotation(&self) -> bool {
        match self {
            Kind::Mini => true,
            _ => false,
        }
    }

    pub fn image_mirror(&self) -> bool {
        match self {
            Kind::Original => true, //Original apparently needs the image mirrored
            _ => false,             //Other kinds untested
        }
    }

    pub fn image_size_bytes(&self) -> usize {
        let (x, y) = self.image_size();
        x * y * 3
    }

    pub(crate) fn image_report_len(&self) -> usize {
        match self {
            Kind::Original => ORIGINAL_IMAGE_REPORT_LEN,
            Kind::Mini => MINI_IMAGE_REPORT_LEN,

            Kind::OriginalV2 => unimplemented!(),
            Kind::Xl => unimplemented!(),
        }
    }

    pub fn image_base(&self) -> &[u8] {
        match self {
            Kind::Original => &ORIGINAL_IMAGE_BASE,
            Kind::Mini => &MINI_IMAGE_BASE,

            Kind::OriginalV2 => unimplemented!(),
            Kind::Xl => unimplemented!(),
        }
    }
}

pub const ORIGINAL_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00,
    0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e,
    0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

const MINI_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00,
    0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e,
    0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

