#[cfg(test)]
mod tests {

    #[test]
    fn test_construction_canvas() {
        let c = ray_tracer::Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels.iter() {
            assert!(pixel.is_equal(ray_tracer::colour(0.0, 0.0, 0.0)));
        }
    }

    #[test]
    fn test_canvas_write_pixel() {
        let mut c = ray_tracer::Canvas::new(10, 20);
        let red = ray_tracer::colour(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert!(c.pixel_at(2, 3).is_equal(red));
    }

    #[test]
    fn test_canvas_ppm_header() {
        let c = ray_tracer::Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let mut ppm_lines = ppm.split("\n");
        assert_eq!(ppm_lines.next().unwrap(), "P3");
        assert_eq!(ppm_lines.next().unwrap(), "5 3");
        assert_eq!(ppm_lines.next().unwrap(), "255");
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    #[test]
    fn test_canvas_ppm_body() {
        let mut c = ray_tracer::Canvas::new(5, 3);
        let c1 = ray_tracer::colour(1.5, 0.0, 0.0);
        let c2 = ray_tracer::colour(0.0, 0.5, 0.0);
        let c3 = ray_tracer::colour(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        assert_eq!(ppm,
"P3
5 3
255
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 128 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255
"
        );
    }
}