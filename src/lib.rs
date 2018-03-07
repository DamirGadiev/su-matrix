mod su_matrix;
#[cfg(test)]
mod tests {
    extern crate num_complex;
    extern crate nalgebra as na;

    use su_matrix;
    use self::num_complex::Complex;
    use self::na::{U2, Matrix, MatrixArray};

    type ComplexMatrix2x2f = Matrix<Complex<f64>, U2, U2, MatrixArray<Complex<f64>, U2, U2>>;

    #[test]
    fn matrix_multiplication() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let m = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let n = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let c = m * n;
        let c00 = c[(0, 0)];
        let c01 = c[(0, 1)];
        let c10 = c[(1, 0)];
        let c11 = c[(1, 1)];
        assert_eq!(c00.re, 0.0);
        assert_eq!(c00.im, 0.0);
        assert_eq!(c01.re, 0.0);
        assert_eq!(c01.im, 2.0);
        assert_eq!(c10.re, 0.0);
        assert_eq!(c10.im, 2.0);
        assert_eq!(c11.re, 0.0);
        assert_eq!(c11.im, 0.0);
    }

    #[test]
    fn matrix_addition() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let m = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let n = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let c = m + n;
        let c00 = c[(0, 0)];
        let c01 = c[(0, 1)];
        let c10 = c[(1, 0)];
        let c11 = c[(1, 1)];
        assert_eq!(c00.re, 2.0);
        assert_eq!(c00.im, 0.0);
        assert_eq!(c01.re, 0.0);
        assert_eq!(c01.im, 2.0);
        assert_eq!(c10.re, 0.0);
        assert_eq!(c10.im, 2.0);
        assert_eq!(c11.re, 2.0);
        assert_eq!(c11.im, 0.0);
    }

    #[test]
    fn matrix_negation() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let m = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let n = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let c = m - n;
        let c00 = c[(0, 0)];
        let c01 = c[(0, 1)];
        let c10 = c[(1, 0)];
        let c11 = c[(1, 1)];
        assert_eq!(c00.re, 0.0);
        assert_eq!(c00.im, 0.0);
        assert_eq!(c01.re, 0.0);
        assert_eq!(c01.im, 0.0);
        assert_eq!(c10.re, 0.0);
        assert_eq!(c10.im, 0.0);
        assert_eq!(c11.re, 0.0);
        assert_eq!(c11.im, 0.0);
    }

    #[test]
    fn matrix_reversion() {
        let a = Complex::new(1.0, 0.0);
        let b = Complex::new(0.0, 1.0);
        let m = ComplexMatrix2x2f::new(a, b,
                                       b, a);
        let c = su_matrix::su_matrix::matrix_inversion(&m);
        let c00 = c[(0, 0)];
        let c01 = c[(0, 1)];
        let c10 = c[(1, 0)];
        let c11 = c[(1, 1)];
        assert_eq!(c00.re, 0.5);
        assert_eq!(c00.im, 0.0);
        assert_eq!(c01.re, 0.0);
        assert_eq!(c01.im, -0.5);
        assert_eq!(c10.re, 0.0);
        assert_eq!(c10.im, -0.5);
        assert_eq!(c11.re, 0.5);
        assert_eq!(c11.im, 0.0);
    }
}
