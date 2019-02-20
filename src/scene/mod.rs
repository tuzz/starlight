use crate::primitive::Primitive;
use crate::light::Light;

struct Scene {
    primitives: Vec<Primitive>,
    lights: Vec<Light>,
}

impl Scene {
    fn new(primitives: Vec<Primitive>, lights: Vec<Light>) -> Self {
        Self { primitives, lights }
    }
}

#[cfg(test)]
mod test;
