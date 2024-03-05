use image::{ImageBuffer, RgbImage};

pub struct Image {
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        let mut height: u32 =
            (width as f64 / aspect_ratio).clamp(u32::MIN as f64, u32::MAX as f64) as u32;
        height = if height < 1 { 1 } else { height };

        Self { width, height }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }
}

impl From<Image> for RgbImage {
    fn from(value: Image) -> Self {
        ImageBuffer::new(value.width, value.height)
    }
}
