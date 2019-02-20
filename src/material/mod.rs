use crate::vector::Vector;

struct Material {
    color: Vector,
}

impl Material {
    fn new(color: Vector) -> Self {
        Self { color }
    }
}

#[cfg(test)]
mod test;
