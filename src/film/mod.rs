use crate::image::Image;

struct Film {
    width: f64,
    height: f64,
    image: Image,
}

impl Film {
    fn new(width: f64, height: f64, image: Image) -> Self {
        Self { width, height, image }
    }
}

#[cfg(test)]
mod test;
