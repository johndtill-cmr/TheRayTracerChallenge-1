#[cfg(test)]
mod tests {

    #[test]
    fn test_ray_construction() {
		let origin = ray_tracer::point(1.0, 2.0, 3.0);
		let direction = ray_tracer::vector(4.0, 5.0, 6.0);
        let m = ray_tracer::Ray { origin: origin, direction: direction };
		assert!(m.origin.is_equal(origin));
		assert!(m.direction.is_equal(direction));
    }
	
	#[test]
	fn test_position() {
		let r = ray_tracer::Ray { origin: ray_tracer::point(2.0, 3.0, 4.0), direction: ray_tracer::vector(1.0, 0.0, 0.0) };
		assert!(r.position(0.0).almost_equal(ray_tracer::point(2.0, 3.0, 4.0)));
		assert!(r.position(1.0).almost_equal(ray_tracer::point(3.0, 3.0, 4.0)));
		assert!(r.position(-1.0).almost_equal(ray_tracer::point(1.0, 3.0, 4.0)));
		assert!(r.position(2.5).almost_equal(ray_tracer::point(4.5, 3.0, 4.0)));
	}
}