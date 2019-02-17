use crate::vector::Vector;

struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Vector>>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = Self::pixels(width, height);

        Self { width, height, pixels }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Vector) {
        self.pixels[y][x] = color;
    }

    fn pixels(width: usize, height: usize) -> Vec<Vec<Vector>> {
        (0..height).map(|_| {
            (0..width).map(|_| {

                Vector::default()

            }).collect()
        }).collect()
    }
}

#[cfg(test)]
mod test;
