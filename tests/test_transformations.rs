#[cfg(test)]
mod tests {

    #[test]
    fn test_translation_point_a() {
        let transform = ray_tracer::translation(5.0, -3.0, 2.0);
        let p = ray_tracer::point(-3.0, 4.0, 5.0);
        assert!((transform * p).almost_equal(ray_tracer::point(2.0, 1.0, 7.0)));
    }

    #[test]
    fn test_translation_point_b() {
        let transform = ray_tracer::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = ray_tracer::point(-3.0, 4.0, 5.0);
        assert!((inv * p).almost_equal(ray_tracer::point(-8.0, 7.0, 3.0)));
    }

    #[test]
    fn test_translation_vector() {
        let transform = ray_tracer::translation(5.0, -3.0, 2.0);
        let v = ray_tracer::vector(-3.0, 4.0, 5.0);
        assert!((transform * v).almost_equal(v));
    }

    #[test]
    fn test_scale_point() {
        let transform = ray_tracer::scaling(2.0, 3.0, 4.0);
        let p = ray_tracer::point(-4.0, 6.0, 8.0);
        assert!((transform * p).almost_equal(ray_tracer::point(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn test_scaling_vector() {
        let transform = ray_tracer::scaling(2.0, 3.0, 4.0);
        let v = ray_tracer::vector(-4.0, 6.0, 8.0);
        assert!((transform * v).almost_equal(ray_tracer::vector(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn test_reflection() {
        let transform = ray_tracer::scaling(-1.0, 1.0, 1.0);
        let p = ray_tracer::point(2.0, 3.0, 4.0);
        assert!((transform * p).almost_equal(ray_tracer::point(-2.0, 3.0, 4.0)));
    }

    #[test]
    fn test_shearing() {
        let transform = ray_tracer::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = ray_tracer::point(2.0, 3.0, 4.0);
        assert!((transform * p).almost_equal(ray_tracer::point(5.0, 3.0, 4.0)));
    }
}