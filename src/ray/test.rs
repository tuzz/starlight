use super::*;

type Subject = Ray;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_ray_with_origin_and_direction() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);

        let subject = Subject::new(origin, direction);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.direction, direction);
    }
}
