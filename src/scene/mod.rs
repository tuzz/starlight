use crate::primitive::Primitive;
use crate::light::Light;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(primitives: Vec<Primitive>, lights: Vec<Light>) -> Self {
        Self { primitives, lights }
    }
}

#[cfg(test)]
mod test;
