pub mod su_matrix {
    extern crate num_complex;
    extern crate nalgebra as na;

    use self::num_complex::Complex;
    use self::na::{U2, Matrix, MatrixArray};

    type ComplexMatrix2x2f = Matrix<Complex<f64>, U2, U2, MatrixArray<Complex<f64>, U2, U2>>;

    /// Calculate the determinant of the matrix.
    pub fn calculate_determinant(matrix: &ComplexMatrix2x2f) -> Complex<f64> {
        let c00 = matrix[(0, 0)];
        let c01 = matrix[(0, 1)];
        let c10 = matrix[(1, 0)];
        let c11 = matrix[(1, 1)];
        c00 * c11 - c01 * c10
    }

    /// Calculate 1/det(A).
    pub fn inverse_determinant(det: &Complex<f64>) -> Complex<f64> {
        let one = Complex::new(1.0, 0.0);
        one / det
    }

    pub fn inverse_sign(a: &Complex<f64>) -> Complex<f64> {
        let minus_one = Complex::new(-1.0, 0.0);
        a * minus_one
    }

    pub fn matrix_inversion(matrix: &ComplexMatrix2x2f) -> ComplexMatrix2x2f {
        // println!("matrix {}", matrix);
        let det = calculate_determinant(&matrix);
        // println!("determinant {}", det);
        let inv_det = inverse_determinant(&det);
        // println!("inversed determinant {}", &inv_det);
        let c00 = matrix[(0, 0)] * inv_det;
        let c01 = matrix[(0, 1)] * inv_det;
        let c10 = matrix[(1, 0)] * inv_det;
        let c11 = matrix[(1, 1)] * inv_det;
        let a = c11;
        let b = inverse_sign(&c01);
        let c = inverse_sign(&c10);
        let d = c00;
        ComplexMatrix2x2f::new(a, b,
                               c, d)
    }

    pub fn matrix_power(power: u64, matrix: &ComplexMatrix2x2f) -> ComplexMatrix2x2f {
        let c00 = matrix[(0, 0)];
        let c01 = matrix[(0, 1)];
        let c10 = matrix[(1, 0)];
        let c11 = matrix[(1, 1)];
        let mut new_matrix = ComplexMatrix2x2f::new(c00, c01,
                                                    c10, c11);
        let mut i:u64 = 1;
        let mut done = false;
        if power == 1 {
            done = true;
        }
        while !done {
            i += 1;
            new_matrix = new_matrix * matrix;
            // println!("Steps: {}", i);
            // println!("New matrix {}", new_matrix);
            if i == power {
                done = true;
            }
        }
        new_matrix
    }
}
