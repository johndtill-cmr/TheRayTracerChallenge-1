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

    #[test]
    fn test_add_tuples() {
        let a1 = ray_tracer::Tuple {x: 3.0, y: -2.0, z: 5.0, w: 1.0};
        let a2 = ray_tracer::Tuple {x: -2.0, y: 3.0, z: 1.0, w: 0.0};
        assert!((a1 + a2).is_equal(ray_tracer::Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0}));
    }

    #[test]
    fn test_subtract_points() {
        let p1 = ray_tracer::point(3.0, 2.0, 1.0);
        let p2 = ray_tracer::point(5.0, 6.0, 7.0);
        assert!((p1 - p2).is_equal(ray_tracer::vector(-2.0, -4.0, -6.0)));
    }

    #[test]
    fn test_subtract_vector_from_points() {
        let p = ray_tracer::point(3.0, 2.0, 1.0);
        let v = ray_tracer::vector(5.0, 6.0, 7.0);
        assert!((p - v).is_equal(ray_tracer::point(-2.0, -4.0, -6.0)));
    }

    #[test]
    fn test_subtract_vectors() {
        let v1 = ray_tracer::vector(3.0, 2.0, 1.0);
        let v2 = ray_tracer::vector(5.0, 6.0, 7.0);
        assert!((v1 - v2).is_equal(ray_tracer::vector(-2.0, -4.0, -6.0)));
    }

    #[test]
    fn test_subtract_from_zero() {
        let zero = ray_tracer::vector(0.0, 0.0, 0.0);
        let v = ray_tracer::vector(1.0, -2.0, 3.0);
        assert!((zero - v).is_equal(ray_tracer::vector(-1.0, 2.0, -3.0)));
    }

    #[test]
    fn test_negate() {
        let a = ray_tracer::Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        assert!((-a).is_equal(ray_tracer::Tuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0}));
    }

    #[test]
    fn test_scalar_mult_a() {
        let a = ray_tracer::Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        assert!((a * 3.5).is_equal(ray_tracer::Tuple{x: 3.5, y: -7.0, z: 10.5, w: -14.0}));
    }

    #[test]
    fn test_scalar_mult_b() {
        let a = ray_tracer::Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        assert!((a * 0.5).is_equal(ray_tracer::Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0}));
    }

    #[test]
    fn test_scalar_div() {
        let a = ray_tracer::Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        assert!((a / 2.0).is_equal(ray_tracer::Tuple{x: 0.5, y: -1.0, z: 1.5, w: -2.0}));
    }

    #[test]
    fn test_magnitude_unit() {
        let a = ray_tracer::vector(1.0, 0.0, 0.0);
        assert_eq!(a.norm(), 1.0);
    }

    #[test]
    fn test_magnitude() {
        let a = ray_tracer::vector(12.0, -5.0, 0.0);
        assert_eq!(a.norm(), 13.0);
    }

    #[test]
    fn test_normalised_unit() {
        let a = ray_tracer::vector(4.0, 0.0, 0.0);
        assert!(a.normalised().is_equal(ray_tracer::vector(1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_normalised() {
        let a = ray_tracer::vector(3.0, 4.0, 0.0);
        assert!(a.normalised().is_equal(ray_tracer::vector(0.6, 0.8, 0.0)));
    }

    #[test]
    fn test_norm_of_normalised() {
        let a = ray_tracer::vector(3.0, 4.0, -5.0);
        assert_eq!(a.normalised().norm(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let a = ray_tracer::vector(1.0, 2.0, 3.0);
        let b = ray_tracer::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0)
    }

    #[test]
    fn test_cross_product() {
        let a = ray_tracer::vector(1.0, 2.0, 3.0);
        let b = ray_tracer::vector(2.0, 3.0, 4.0);
        assert!(a.cross(&b).is_equal(ray_tracer::vector(-1.0, 2.0, -1.0)));
        assert!(b.cross(&a).is_equal(ray_tracer::vector(1.0, -2.0, 1.0)));
    }
}