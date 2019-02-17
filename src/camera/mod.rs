use crate::vector::Vector;
use crate::film::Film;

struct Camera {
    origin: Vector,
    direction: Vector,
    orientation: Vector,
    film: Film,
}

impl Camera {
    fn new(origin: Vector, direction: Vector, orientation: Vector, film: Film) -> Self {
        Self { origin, direction, orientation, film }
    }
}

#[cfg(test)]
mod test;
