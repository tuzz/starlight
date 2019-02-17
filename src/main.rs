mod vector;
mod ray;
mod image;
mod film;
mod camera;
mod sphere;

use vector::Vector;
use image::Image;
use film::Film;
use camera::Camera;

fn main() {
    let origin = Vector::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let orientation = Vector::new(0.0, 1.0, 0.0);
    let image = Image::new(800, 600);
    let film = Film::new(8.0, 6.0, image);
    let mut camera = Camera::new(origin, direction, orientation, film);
    let purple = Vector::new(0.5, 0.0, 0.5);

    camera.trace_rays(|_ray| {
        purple
    });

    camera.take_photograph("hello.png");
}
