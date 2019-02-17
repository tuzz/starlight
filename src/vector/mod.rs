struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test;
