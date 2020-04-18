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

/// Stream Deck Image Modes
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColourOrder {
    RGB,
    BGR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mirroring {
    None,
    X,
    Y,
    Both,
}

const ORIGINAL_IMAGE_REPORT_LEN: usize = 8191;
const MINI_IMAGE_REPORT_LEN: usize = 1024;
const ORIGINAL_V2_IMAGE_REPORT_LEN: usize = 1024;
const XL_IMAGE_REPORT_LEN: usize = 1024;

const ORIGINAL_IMAGE_REPORT_HEADER_LEN: usize = 16;
const MINI_IMAGE_REPORT_HEADER_LEN: usize = 16;
const ORIGINAL_V2_IMAGE_REPORT_HEADER_LEN: usize = 8;
const XL_IMAGE_REPORT_HEADER_LEN: usize = 8;

impl Kind {
    pub fn keys(&self) -> u8 {
        match self {
            Kind::Original | Kind::OriginalV2 => 15,
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

    pub fn image_rotation(&self) -> Rotation {
        match self {
            Kind::Mini => Rotation::Rot270,
            _ => Rotation::Rot0,
        }
    }

    pub fn image_mirror(&self) -> Mirroring {
        match self {
            // Mini has rotation, not mirror
            Kind::Mini => Mirroring::None,
            // On the original the image is flipped across the Y axis
            Kind::Original => Mirroring::Y,
            // On the V2 devices, both X and Y need to flip
            Kind::OriginalV2 | Kind::Xl => Mirroring::Both,
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
            Kind::OriginalV2 => ORIGINAL_V2_IMAGE_REPORT_LEN,
            Kind::Xl => XL_IMAGE_REPORT_LEN,
        }
    }

    pub(crate) fn image_report_header_len(&self) -> usize {
        match self {
            Kind::Original => ORIGINAL_IMAGE_REPORT_HEADER_LEN,
            Kind::Mini => MINI_IMAGE_REPORT_HEADER_LEN,
            Kind::OriginalV2 => ORIGINAL_V2_IMAGE_REPORT_HEADER_LEN,
            Kind::Xl => XL_IMAGE_REPORT_HEADER_LEN,
        }
    }

    pub fn image_base(&self) -> &[u8] {
        match self {
            Kind::Original => &ORIGINAL_IMAGE_BASE,
            Kind::Mini => &MINI_IMAGE_BASE,

            Kind::OriginalV2 | Kind::Xl => &[],
        }
    }

    pub(crate) fn image_colour_order(&self) -> ColourOrder {
        match self {
            Kind::Original | Kind::Mini => ColourOrder::BGR,
            Kind::OriginalV2 | Kind::Xl => ColourOrder::RGB,
        }
    }

    pub(crate) fn is_v2(&self) -> bool {
        match self {
            Kind::OriginalV2 | Kind::Xl => true,
            _ => false,
        }
    }
}

pub const ORIGINAL_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const MINI_IMAGE_BASE: [u8; 54] = [
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
