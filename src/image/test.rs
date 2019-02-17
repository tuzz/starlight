use super::*;

type Subject = Image;

mod new {
    use super::*;

    fn it_builds_an_image_with_a_two_dimensional_array_of_pixel_colors() {
        let subject = Image::new(800, 600);

        assert_eq!(subject.width, 800);
        assert_eq!(subject.height, 600);

        assert_eq!(subject.pixels[0].len(), 800);
        assert_eq!(subject.pixels.len(), 600);
    }

    fn it_sets_the_pixel_colors_to_black() {
        let subject = Image::new(800, 600);

        assert_eq!(subject.pixels[0][0], Vector::new(0.0, 0.0, 0.0));
    }
}

mod set {
    use super::*;

    fn it_sets_the_color_of_a_pixel() {
        let mut subject = Image::new(800, 600);
        let purple = Vector::new(0.5, 0.0, 0.5);

        let x = 0;
        let y = 1;

        subject.set(x, y, purple);

        assert_eq!(subject.pixels[x][y], purple);
    }
}

mod write {
    use super::*;

    #[test]
    fn it_writes_the_image_to_a_file() {
        let mut subject = Image::new(500, 500);
        let purple = Vector::new(0.5, 0.0, 0.5);

        for i in 0..500 {
            subject.set(i, i, purple);
        }

        subject.write("test-image.png");
    }
}
