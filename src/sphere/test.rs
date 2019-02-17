use super::*;

type Subject = Sphere;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_sphere() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let purple = Vector::new(0.5, 0.0, 0.5);

        let subject = Subject::new(origin, 1.5, purple);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.radius, 1.5);
        assert_eq!(subject.color, purple);
    }
}
