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
        let color = Vector::new(0.1, 0.2, 0.3);

        let x = 0;
        let y = 1;

        subject.set(x, y, color);

        assert_eq!(subject.pixels[x][y], color);
    }
}
