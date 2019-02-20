mod vector;
mod ray;
mod image;
mod film;
mod camera;
mod sphere;
mod interaction;
mod light;
mod material;
mod primitive;
mod scene;

use vector::Vector;
use image::Image;
use film::Film;
use camera::Camera;
use sphere::Sphere;
use material::Material;
use primitive::Primitive;

fn main() {
    let origin = Vector::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let orientation = Vector::new(0.0, 1.0, 0.0);
    let image = Image::new(800, 600);
    let film = Film::new(8.0, 6.0, image);
    let mut camera = Camera::new(origin, direction, orientation, film);
    let purple = Vector::new(0.5, 0.0, 0.5);
    let aubergine = Material::new(purple);
    let sphere = Sphere::new(Vector::new(0.0, 0.0, 5.0), 4.0);
    let primitive = Primitive::new(sphere, aubergine);

    camera.trace_rays(|ray| {
        if let Some(interaction) = primitive.intersection(ray) {
            return primitive.material.color * interaction.ray_t;
        }

        Vector::new(0.2, 0.2, 0.2)
    });

    camera.take_photograph("hello.png");
}
