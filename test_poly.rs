use ark_poly::{DenseUVPolynomial, univariate::DensePolynomial};
use ark_ff::PrimeField;

fn test_division<F: PrimeField>() {
    let a = DensePolynomial::from_coefficients_vec(vec![F::from(1u64), F::from(2u64)]);
    let b = DensePolynomial::from_coefficients_vec(vec![F::from(1u64), F::from(1u64)]);
    
    // Check what methods are available
    println!("Methods on DensePolynomial:");
    // This will fail to compile but show available methods
    // a.divide_by(&b);
}
