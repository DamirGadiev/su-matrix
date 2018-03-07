// This module should implement elements of the Special Unitary group.
// SU group consists from unitary matrices with determinant equal to 1 and
// with multiplication as group operation. For the matrix with dimensions
// of n x n size is written as SU(n).
mod su_matrix;

extern crate num_traits;
extern crate num_complex;
extern crate nalgebra as na;

use num_complex::Complex;
use na::{U2, Matrix, MatrixArray};
use su_matrix::su_matrix::matrix_inversion;
use su_matrix::su_matrix::matrix_power;

type ComplexMatrix2x2f = Matrix<Complex<f64>, U2, U2, MatrixArray<Complex<f64>, U2, U2>>;

fn calculate_resistance_fixed_a(length: f64, m: f64, a: f64, plank_const: f64, energy: f64, potential: f64, n: u64) ->f64 {
    // 2mE
    let two_m_e = 2.0 * m * energy;
    // ((2mE)^1/2)/h
    let k = two_m_e.sqrt() / plank_const;
    // Pfi parameter.
    let phi = k * a;
    // E^2
    let e_square = energy * energy;
    // m*(omega^2)
    let m_omega_square = m * potential * potential;
    // 2 * h * h * E.
    let two_plank_square_energy = 2.0 * plank_const * plank_const * energy;
    // Khi^2
    let khi_square = m_omega_square / two_plank_square_energy;
    // Khi
    let khi = khi_square.sqrt() * 0.0000000000001 * 0.00001;
    // Resistance parameter.
    let two_pi_h_div_e = 2.0 * 3.14 * plank_const / e_square;
    // A - Matrix.
    let a00 = Complex::new(1.0, khi);
    let a01 = Complex::new(0.0, khi);
    let a10 = Complex::new(0.0, -khi);
    let a11 = Complex::new(1.1, -khi);
    let a_matrix = ComplexMatrix2x2f::new(a00, a01,
                                         a10, a11);
    // println!("A Matrix: {}", AMatrix);
    // B - matrix
    let b00 = Complex::new(0.0, -phi).exp();
    let b01 = Complex::new(0.0, 0.0);
    let b10 = Complex::new(0.0, 0.0);
    let b11 = Complex::new(0.0, phi).exp();
    let b_matrix = ComplexMatrix2x2f::new(b00, b01,
                                         b10, b11);
    // println!("B Matrix: {}", BMatrix);
    let b_matrix_inverted = matrix_inversion(&b_matrix);
    // println!("B Matrix Inverted: {}", BMatrixInverted);
    let a_b_matrix = a_matrix * b_matrix;
    // println!("AB Matrix: {}", ABMatrix);
    let power_matrix = matrix_power(n, &a_b_matrix);
    //  println!("Power Matrix: {}", powerMatrix);
    let transfer_matrix = power_matrix * b_matrix_inverted;
    // println!("Transfer Matrix: {}", powerMatrix);
    let ro = two_pi_h_div_e * transfer_matrix[(1, 0)].norm_sqr();
    ro
}

fn calculate_resistance_from_a() {
    let n = 1000.0;
    let mut a_null = 0.0;
    let a_step = 0.00000001;
    let a_final = n * a_step;
    let n_int = n as u64;
    let mut done = false;
    while !done {
        a_null = a_null + a_step;
        let length = n * a_null;
        let m = 9.10938356 * 0.0000000000000000000000000000001;
        let plank_const = 1.054571800 * 0.00000000000000000000000000000001;
        let energy = 10.6; //* 0.0000000000000001;
        let potential = 1.6 * 0.0000000000000001;
        if a_null >= a_final {
            done = true;
        }
        let ro = calculate_resistance_fixed_a(length, m, a_null, plank_const, energy, potential, n_int);
        println!("{}  {}", a_null, ro);
    }
    // println!("Length {} meters", a_null);
}

fn calculate_resistance_from_energy() {
    // println!("a:ro");
    let n = 1000.0;
    let a = 0.00000001;
    let mut e_null = 0.0;
    let e_step = 0.001;
    let e_final = 10.0;
    let n_int = n as u64;
    let mut done = false;
    while !done {
        e_null = e_null + e_step;
        let length = n * a;
        let m = 9.10938356 * 0.0000000000000000000000000000001;
        let plank_const = 1.054571800 * 0.00000000000000000000000000000001;
        let potential = 1.6 * 0.0000000000000001;
        if e_null >= e_final {
            done = true;
        }
        let ro = calculate_resistance_fixed_a(length, m, a, plank_const, e_null, potential, n_int);
        println!("{}  {}", e_null, ro);
    }
    // println!("Length {} meters", a_null);
}

fn calculate_resistance_from_potential() {
    // println!("a:ro");
    let n = 1000.0;
    let a = 0.00000001;
    let mut p_null = 0.0;
    let p_step = 0.001;
    let p_final = 10.0;
    let n_int = n as u64;
    let mut done = false;
    while !done {
        p_null = p_null + p_step;
        let length = n * a;
        let m = 9.10938356 * 0.0000000000000000000000000000001;
        let plank_const = 1.054571800 * 0.00000000000000000000000000000001;
        let energy = 1.6; //* 0.0000000000000001;
        if p_null >= p_final {
            done = true;
        }
        let ro = calculate_resistance_fixed_a(length, m, a, plank_const, energy, p_null, n_int);
        println!("{}  {}", p_null, ro);
    }
    // println!("Length {} meters", a_null);
}

fn main() {
    calculate_resistance_from_a();
    // calculate_resistance_from_potential();
}
