use super::*;

type Subject = Interaction;

mod new {
    use super::*;

    #[test]
    fn it_builds_an_interaction_containing_the_rays_parameter_and_a_surface_normal() {
        let ray_t = 1.23;
        let normal = Vector::new(1.0, 2.0, 3.0);

        let subject = Subject::new(ray_t, normal);

        assert_eq!(subject.ray_t, ray_t);
        assert_eq!(subject.normal, normal);
    }
}
