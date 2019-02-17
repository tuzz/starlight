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

mod addition {
    use super::*;

    #[test]
    fn it_adds_the_vectors_components() {
        let a = Subject::new(1.0, 2.0, 3.0);
        let b = Subject::new(4.0, 5.0, 6.0);

        let subject = a + b;

        assert_eq!(subject, Subject::new(5.0, 7.0, 9.0));
    }
}

mod subtraction {
    use super::*;

    #[test]
    fn it_subtracts_the_vectors_components() {
        let a = Subject::new(5.0, 5.0, 5.0);
        let b = Subject::new(1.0, 2.0, 3.0);

        let subject = a - b;

        assert_eq!(subject, Subject::new(4.0, 3.0, 2.0));
    }
}
