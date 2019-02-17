use crate::vector::Vector;

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Vector>>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        let pixels = Self::pixels(width, height);

        Self { width, height, pixels }
    }

    fn pixels(width: u32, height: u32) -> Vec<Vec<Vector>> {
        (0..height).map(|_| {
            (0..width).map(|_| {

                Vector::default()

            }).collect()
        }).collect()
    }
}

#[cfg(test)]
mod test;
