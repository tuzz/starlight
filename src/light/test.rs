use super::*;

type Subject = Light;

mod new {
    use super::*;

    #[test]
    fn it_builds_a_light_with_origin_and_power() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let power = 5.0;

        let subject = Subject::new(origin, power);

        assert_eq!(subject.origin, origin);
        assert_eq!(subject.power, power);
    }
}
