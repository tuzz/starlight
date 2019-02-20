use super::*;
use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::material::Material;

type Subject = Scene;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_scene_with_primitives_and_lights() {
        let sphere = Sphere::new(Vector::default(), 1.0);
        let aubergine = Material::new(Vector::new(0.5, 0.0, 0.5));
        let primitive = Primitive::new(sphere, aubergine);
        let light = Light::new(Vector::default(), 5.0);

        let subject = Subject::new(vec![primitive], vec![light]);

        assert_eq!(subject.primitives, &[primitive]);
        assert_eq!(subject.lights, &[light]);
    }
}
