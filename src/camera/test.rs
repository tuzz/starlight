use super::*;
use crate::image::Image;

type Subject = Camera;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_camera_and_places_it_in_the_scene() {
        let origin = Vector::default();
        let direction = Vector::new(0.0, 0.0, 1.0);
        let orientation = Vector::new(0.0, 1.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film.clone());

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.direction, direction);
        assert_eq!(subject.orientation, orientation);
        assert_eq!(subject.film, film);
    }

    #[test]
    fn it_normalizes_the_direction_and_orientation_vectors() {
        let origin = Vector::default();
        let direction = Vector::new(0.0, 0.0, 2.0);
        let orientation = Vector::new(0.0, 2.0, 0.0);

        let image = Image::new(200, 100);
        let film = Film::new(2.0, 1.0, image);

        let subject = Subject::new(origin, direction, orientation, film.clone());

        assert_eq!(subject.direction, Vector::new(0.0, 0.0, 1.0));
        assert_eq!(subject.orientation, Vector::new(0.0, 1.0, 0.0));
    }
}
