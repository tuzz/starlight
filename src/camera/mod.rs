use crate::vector::Vector;
use crate::film::Film;

#[derive(Default)]
struct Camera {
    origin: Vector,
    direction: Vector,
    orientation: Vector,
    film: Film,

    left_to_right: Vector,
    top_to_bottom: Vector,
}

impl Camera {
    fn new(origin: Vector, direction: Vector, orientation: Vector, film: Film) -> Self {
        let direction = direction.normalize();
        let orientation = orientation.normalize();

        Self { origin, direction, orientation, film, ..Self::default() }.set_spans()
    }

    fn set_spans(mut self) -> Self {
        let up = self.orientation;
        let down = -up;
        let left = self.direction * self.orientation;
        let right = -left;

        self.left_to_right = right * self.film.width;
        self.top_to_bottom = down * self.film.height;

        self
    }
}

#[cfg(test)]
mod test;
