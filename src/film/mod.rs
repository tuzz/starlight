use crate::image::Image;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Film {
    pub width: f64,
    pub height: f64,

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

    pub fn pixel_ratios(&self) -> Vec<(usize, usize, f64, f64)> {
        (0..self.image.width).flat_map(|x| {
            (0..self.image.height).map(move |y| {
                let (x_ratio, y_ratio) = self.pixel_ratio(x, y);

                (x, y, x_ratio, y_ratio)
            })
        }).collect()
    }

    fn pixel_ratio(&self, x: usize, y: usize) -> (f64, f64) {
        (x as f64 * self.pixel_width, y as f64 * self.pixel_height)
    }
}

#[cfg(test)]
mod test;
