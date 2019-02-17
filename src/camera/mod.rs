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
        let direction = direction.normalize();
        let orientation = orientation.normalize();

        Self { origin, direction, orientation, film }
    }
}

#[cfg(test)]
mod test;
