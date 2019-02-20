use crate::vector::Vector;

struct Light {
    origin: Vector,
    power: f64,
}

impl Light {
    fn new(origin: Vector, power: f64) -> Self {
        Self { origin, power }
    }
}

#[cfg(test)]
mod test;
