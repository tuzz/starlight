use crate::image::Image;

#[derive(Debug, Clone, PartialEq)]
pub struct Film {
    width: f64,
    height: f64,
    image: Image,

    pixel_width: f64,
    pixel_height: f64,
}

impl Film {
    pub fn new(width: f64, height: f64, image: Image) -> Self {
        let pixel_width = width / image.width as f64;
        let pixel_height = height / image.height as f64;

        Self { width, height, image, pixel_width, pixel_height }
    }

    pub fn pixel_ratio(&self, x: u32, y: u32) -> (f64, f64) {
        (x as f64 * self.pixel_width, y as f64 * self.pixel_height)
    }
}

#[cfg(test)]
mod test;
