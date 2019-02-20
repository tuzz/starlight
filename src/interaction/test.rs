use super::*;

type Subject = Interaction;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_interaction_containing_the_rays_parameter_and_a_surface_normal() {
        let ray_t = 1.23;
        let normal = Vector::new(1.0, 0.0, 0.0);

        let subject = Subject::new(ray_t, normal);

        assert_eq!(subject.ray_t, ray_t);
        assert_eq!(subject.normal, normal);
    }

    #[test]
    fn it_normalizes_the_surface_normal() {
        let normal = Vector::new(2.0, 0.0, 0.0);
        let subject = Subject::new(1.23, normal);

        assert_eq!(subject.normal, Vector::new(1.0, 0.0, 0.0));
    }
}
