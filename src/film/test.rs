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

mod map_pixels {
    use super::*;

    #[test]
    fn it_sets_the_color_of_each_pixel_from_the_closures_return_value() {
        let image = Image::new(200, 100);
        let mut subject = Subject::new(2.0, 1.0, image);
        let purple = Vector::new(0.5, 0.0, 0.5);

        subject.map_pixels(|_| purple);

        assert_eq!(subject.image.get(0, 0), purple);
        assert_eq!(subject.image.get(199, 99), purple);
    }

    #[test]
    fn it_calls_the_closure_with_each_pixels_x_and_y_ratios() {
        let image = Image::new(1, 1);
        let mut subject = Subject::new(2.0, 1.0, image);

        subject.map_pixels(|(x_ratio, y_ratio)| {
            assert_eq!(x_ratio, 0.0);
            assert_eq!(y_ratio, 0.0);

            Vector::default()
        });
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
