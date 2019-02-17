use crate::vector::Vector;

struct Sphere {
    origin: Vector,
    radius: f64,
    color: Vector,
}

impl Sphere {
    fn new(origin: Vector, radius: f64, color: Vector) -> Self {
        Self { origin, radius, color }
    }
}

#[cfg(test)]
mod test;
