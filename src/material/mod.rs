use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Vector,
}

impl Material {
    pub fn new(color: Vector) -> Self {
        Self { color }
    }
}

#[cfg(test)]
mod test;
