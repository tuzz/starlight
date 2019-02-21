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
mod integrator;

use vector::Vector;
use image::Image;
use film::Film;
use camera::Camera;
use sphere::Sphere;
use material::Material;
use primitive::Primitive;
use light::Light;
use scene::Scene;
use integrator::Integrator;

fn main() {
    let red = Vector::new(1.0, 0.5, 0.5);
    let green = Vector::new(0.0, 1.0, 0.5);
    let white = Vector::new(1.0, 1.0, 1.0);
    let yellow = Vector::new(0.7, 0.6, 0.3);

    let sphere1 = Sphere::new(Vector::new(-5.0, 0.0, 15.0), 5.0);
    let mut primitive1 = Primitive::new(sphere1, Material::new(red));
    primitive1.material.reflectance = 1.0;

    let sphere2 = Sphere::new(Vector::new(5.0, 0.0, 15.0), 5.0);
    let mut primitive2 = Primitive::new(sphere2, Material::new(green));
    primitive2.material.reflectance = 1.0;

    let sphere3 = Sphere::new(Vector::new(0.0, 8.0, 17.0), 4.0);
    let mut primitive3 = Primitive::new(sphere3, Material::new(white));
    primitive3.material.reflectance = 1.0;

    let sphere4 = Sphere::new(Vector::new(0.0, -7.0, 12.0), 4.0);
    let mut primitive4 = Primitive::new(sphere4, Material::new(yellow));
    primitive4.material.reflectance = 1.0;

    let light1 = Light::new(Vector::new(0.0, 5.0, 10.0), 10.0);
    let light2 = Light::new(Vector::new(0.0, -5.0, 10.0), 3.0);
    let light3 = Light::new(Vector::new(-4.0, -5.0, 10.0), 10.0);
    let light4 = Light::new(Vector::new(3.0, 5.0, 10.0), 10.0);
    let light5 = Light::new(Vector::new(0.0, 0.1, 0.0), 20.0);

    let scene = Scene::new(
        vec![primitive1, primitive2, primitive3, primitive4],
        vec![light1, light2, light3, light4, light5],
    );

    let origin = Vector::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);
    let orientation = Vector::new(0.0, 1.0, 0.0);
    let image = Image::new(3840, 2160);
    let film = Film::new(1.92, 1.08, image);
    let camera = Camera::new(origin, direction, orientation, film);

    Integrator::new(camera).render(scene);
}
