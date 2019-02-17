use crate::vector::Vector;

struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }
}

#[cfg(test)]
mod test;
