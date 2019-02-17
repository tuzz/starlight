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

mod pixel_ratios {
    use super::*;

    #[test]
    fn it_returns_an_array_of_all_pixels_with_their_x_y_ratios() {
        let image = Image::new(2, 2);
        let mut subject = Subject::new(2.0, 1.0, image);

        assert_eq!(subject.pixel_ratios(), &[
            (0, 0, 0.0, 0.0),
            (0, 1, 0.0, 0.5),
            (1, 0, 1.0, 0.0),
            (1, 1, 1.0, 0.5),
        ]);
    }
}

mod pixel_ratio {
    use super::*;

    #[test]
    fn it_returns_the_ratio_of_the_pixels_position_to_the_films_dimensions() {
        let image = Image::new(200, 100);
        let subject = Subject::new(2.0, 1.0, image);

        assert_eq!(subject.pixel_ratio(0, 0), (0.0, 0.0));

        assert_eq!(subject.pixel_ratio(1, 0), (0.01, 0.0));
        assert_eq!(subject.pixel_ratio(2, 0), (0.02, 0.0));

        assert_eq!(subject.pixel_ratio(0, 1), (0.0, 0.01));
        assert_eq!(subject.pixel_ratio(0, 2), (0.0, 0.02));

        assert_eq!(subject.pixel_ratio(199, 99), (1.99, 0.99));
    }
}
