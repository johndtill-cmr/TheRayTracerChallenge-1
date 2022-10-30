#[cfg(test)]
mod tests {

    #[test]
    fn test_construction_colour() {
        let a = ray_tracer::colour(-0.5, 0.4, 1.7);
        assert_eq!(a.red(), -0.5);
        assert_eq!(a.green(), 0.4);
        assert_eq!(a.blue(), 1.7);
    }

    #[test]
    fn test_colour_addition() {
        let c1 = ray_tracer::colour(0.9, 0.6, 0.75);
        let c2 = ray_tracer::colour(0.7, 0.1, 0.25);
        assert!((c1 + c2).almost_equal(ray_tracer::colour(1.6, 0.7, 1.0)));
    }

    #[test]
    fn test_colour_subtraction() {
        let c1 = ray_tracer::colour(0.9, 0.6, 0.75);
        let c2 = ray_tracer::colour(0.7, 0.1, 0.25);
        assert!((c1 - c2).almost_equal(ray_tracer::colour(0.2, 0.5, 0.5)));
    }

    #[test]
    fn test_colour_hadamard_prod() {
        let c1 = ray_tracer::colour(1.0, 0.2, 0.4);
        let c2 = ray_tracer::colour(0.9, 1.0, 0.1);
        assert!(c1.hadamard_prod(&c2).almost_equal(ray_tracer::colour(0.9, 0.2, 0.04)));
    }
}