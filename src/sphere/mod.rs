use crate::vector::Vector;
use crate::ray::Ray;
use crate::interaction::Interaction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub origin: Vector,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Vector, radius: f64) -> Self {
        Self { origin, radius }
    }

    pub fn intersection(&self, ray: Ray) -> Option<Interaction> {
        // Calculate the vector from the center of the sphere to the ray's origin.
        let center_to_ray = ray.origin - self.origin;

        // Calculate the coefficients of the quadratic rearrangement:
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * center_to_ray.dot(ray.direction);
        let c = center_to_ray.dot(center_to_ray) - self.radius * self.radius;

        // Calculate the term under the square root of the quadratic equation:
        let discriminant = b * b - 4.0 * a * c;

        // If the discriminant is negative, there are no intersections:
        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();

        // Find both solutions of the quadratic equation:
        let t0 = (-b - sqrt) / (a * 2.0);
        let t1 = (-b + sqrt) / (a * 2.0);

        // The intersection is the smallest positive solution:
        let ray_t = if t0 > 0.0 { t0 } else { t1 };
        let normal = ray.at(ray_t) - self.origin;

        Some(Interaction::new(ray_t, normal))
    }
}

#[cfg(test)]
mod test;
