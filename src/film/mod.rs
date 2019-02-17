use crate::image::Image;

#[derive(Debug, Clone, PartialEq)]
pub struct Film {
    width: f64,
    height: f64,
    image: Image,
}

impl Film {
    pub fn new(width: f64, height: f64, image: Image) -> Self {
        Self { width, height, image }
    }
}

#[cfg(test)]
mod test;
