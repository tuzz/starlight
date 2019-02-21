use crate::scene::Scene;
use crate::camera::Camera;
use crate::vector::Vector;
use crate::ray::Ray;

pub struct Integrator {
    camera: Camera,
}

impl Integrator {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn render(&mut self, scene: Scene) {
        self.camera.trace_rays(|ray| Self::li(ray, &scene));

        self.camera.take_photograph("render.png");
    }

    fn li(ray: Ray, scene: &Scene) -> Vector {
        let intersection = scene.intersection(ray);

        // If the ray doesn't intersect, set the background to gray.
        if intersection.is_none() {
            return Vector::new(0.2, 0.2, 0.2);
        }

        // Get details of the intersection, e.g. surface normal, primitive
        let (interaction, primitive) = intersection.unwrap();

        let total_radiance = scene.lights.iter().map(|light| {
            // Get the amount of light falling on the point of intersection.
            let (radiance, incident) = light.sample_li(&interaction);

            // Build a ray from the point of intersection to the light.
            let shadow_ray = Ray::new(interaction.origin, incident);

            // If the light is occluded, don't add its radiance.
            if let Some(_) = scene.intersection(shadow_ray) {
                return 0.0;
            }

            // Calculate the proportion of light falling on the tilted surface.
            let cosine = incident.dot(interaction.normal).abs();

            radiance * cosine
        }).sum::<f64>();

        primitive.material.color * total_radiance
    }
}

#[cfg(test)]
mod test;
