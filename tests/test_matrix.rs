#[cfg(test)]
mod tests {

    #[test]
    fn test_construction_matrix_4x4() {
        let m = ray_tracer::Matrix::new(4, 4, [1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5].to_vec());
        assert_eq!(m[[0, 0]], 1.0);
        assert_eq!(m[[0, 3]], 4.0);
        assert_eq!(m[[1, 0]], 5.5);
        assert_eq!(m[[1, 2]], 7.5);
        assert_eq!(m[[2, 2]], 11.0);
        assert_eq!(m[[3, 0]], 13.5);
        assert_eq!(m[[3, 2]], 15.5);
    }

    #[test]
    fn test_construction_matrix_2x2() {
        let m = ray_tracer::Matrix::new(2, 2, [-3.0, 5.0, 1.0, -2.0].to_vec());
        assert_eq!(m[[0, 0]], -3.0);
        assert_eq!(m[[0, 1]], 5.0);
        assert_eq!(m[[1, 1]], -2.0);
    }

    #[test]
    fn test_construction_matrix_3x3() {
        let m = ray_tracer::Matrix::new(3, 3, [-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0].to_vec());
        assert_eq!(m[[0, 0]], -3.0);
        assert_eq!(m[[1, 1]], -2.0);
        assert_eq!(m[[2, 2]], 1.0);
    }

    #[test]
    fn test_equal_matrices() {
        let a = ray_tracer::Matrix::new(2, 2, [1.0, 2.0, 5.0, 6.0].to_vec());
        let b = ray_tracer::Matrix::new(2, 2, [1.0, 2.0, 5.0, 6.0].to_vec());
        assert!(a.almost_equal(b));
    }

    #[test]
    fn test_unequal_matrices() {
        let a = ray_tracer::Matrix::new(2, 2, [1.0, 2.0, 5.0, 6.0].to_vec());
        let b = ray_tracer::Matrix::new(2, 2, [2.0, 3.0, 6.0, 7.0].to_vec());
        assert!(!a.almost_equal(b));
    }

    #[test]
    fn test_multiply_matrices() {
        let a = ray_tracer::Matrix::new(4, 4, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0].to_vec());
        let b = ray_tracer::Matrix::new(4, 4, [-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0].to_vec());
        assert!((a*b).almost_equal(ray_tracer::Matrix::new(4, 4,
            [20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0].to_vec())));
    }

    #[test]
    fn test_multiply_tuple() {
        let a = ray_tracer::Matrix::new(4, 4, [1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0].to_vec());
        let b = ray_tracer::Tuple {x: 1.0, y: 2.0, z: 3.0, w: 1.0};
        assert!((a*b).almost_equal(ray_tracer::Tuple {x: 18.0, y: 24.0, z: 33.0, w: 1.0}));
    }

    #[test]
    fn test_multiply_identity() {
        let a = ray_tracer::Matrix::new(4, 4, [1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0].to_vec());
        assert!((a.clone()*ray_tracer::Matrix::identity(4, 4)).almost_equal(a));
    }

    #[test]
    fn test_transpose() {
        let a = ray_tracer::Matrix::new(4, 4, [0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0].to_vec());
        assert!(a.transpose().almost_equal(ray_tracer::Matrix::new(
            4, 4, [0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0].to_vec())));
    }

    #[test]
    fn test_determinant_2x2() {
        let a = ray_tracer::Matrix::new(2, 2, [1.0, 5.0, -3.0, 2.0].to_vec());
        assert_eq!(a.det2x2(), 17.0);
    }

    #[test]
    fn test_submatrix_3x3() {
        let a = ray_tracer::Matrix::new(3, 3, [1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0].to_vec());
        assert!(a.submatrix(0, 2).almost_equal(ray_tracer::Matrix::new(2, 2, [-3.0, 2.0, 0.0, 6.0].to_vec())));
    }

    #[test]
    fn test_submatrix_4x4() {
        let a = ray_tracer::Matrix::new(4, 4, [-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0].to_vec());
        assert!(a.submatrix(2, 1).almost_equal(ray_tracer::Matrix::new(3, 3, [-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0].to_vec())));
    }

    #[test]
    fn test_minor_3x3() {
        let a = ray_tracer::Matrix::new(3, 3, [3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0].to_vec());
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor_3x3() {
        let a = ray_tracer::Matrix::new(3, 3, [3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0].to_vec());
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_3x3() {
        let a = ray_tracer::Matrix::new(3, 3, [1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0].to_vec());
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.det(), -196.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let a = ray_tracer::Matrix::new(4, 4, [-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0].to_vec());
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.det(), -4071.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn test_non_invertible() {
        let a = ray_tracer::Matrix::new(4, 4, [-4.0, -2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0].to_vec());
        assert_eq!(a.det(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn test_inverse() {
        let a = ray_tracer::Matrix::new(4, 4, [-5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0].to_vec());
        assert!(a.is_invertible());
        assert!(a.inverse().almost_equal(ray_tracer::Matrix::new(4, 4,
            [ 0.21805,  0.45113,  0.24060, -0.04511,
             -0.80827, -1.45677, -0.44361,  0.52068,
             -0.07895, -0.22368, -0.05263,  0.19737,
             -0.52256, -0.81391, -0.30075,  0.30639].to_vec())));
    }
}