use crate::vector::Vector;

#[derive(Debug, PartialEq)]
pub struct Interaction {
    pub ray_t: f64,
    pub normal: Vector,
}

impl Interaction {
    pub fn new(ray_t: f64, normal: Vector) -> Self {
        let normal = normal.normalize();

        Self { ray_t, normal }
    }
}

#[cfg(test)]
mod test;
