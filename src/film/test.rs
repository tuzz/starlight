use super::*;

type Subject = Film;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_film_that_contains_an_image_of_the_scene() {
        let image = Image::new(200, 100);
        let subject = Subject::new(2.0, 1.0, image.clone());

        assert_eq!(subject.image, image);
    }
}
