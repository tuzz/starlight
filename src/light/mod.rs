use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    origin: Vector,
    power: f64,
}

impl Light {
    pub fn new(origin: Vector, power: f64) -> Self {
        Self { origin, power }
    }
}

#[cfg(test)]
mod test;
