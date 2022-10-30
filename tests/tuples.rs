#[cfg(test)]
mod tests {

    #[test]
    fn test_construction_first_scenario() {
        let a = ray_tracer::Tuple {x: 4.3, y: -4.2, z: 3.1, w: 1.0};
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn test_construction_second_scenario() {
        let a = ray_tracer::Tuple {x: 4.3, y: -4.2, z: 3.1, w: 0.0};
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn test_vector_factory() {
        let a = ray_tracer::vector(4.3, -4.2, 3.1);
        assert!(a.is_vector());
    }

    #[test]
    fn test_point_factory() {
        let a = ray_tracer::point(4.3, -4.2, 3.1);
        assert!(a.is_point());
    }
}