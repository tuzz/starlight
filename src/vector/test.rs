use super::*;

type Subject = Vector;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_vector_with_x_y_and_z_coordinates() {
        let subject = Subject::new(1.0, 2.0, 3.0);

        assert_eq!(subject.x, 1.0);
        assert_eq!(subject.y, 2.0);
        assert_eq!(subject.z, 3.0);
    }
}
