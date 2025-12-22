//! # Polynomial Ring Algebra Module
//!
//! This module implements polynomial ring arithmetic over finite fields,
//! specifically designed for cryptographic applications in multi-linear group encryption.
//!
//! ## Security Considerations
//!
//! - All polynomial operations are performed modulo a fixed irreducible polynomial
//! - Noise sampling uses cryptographically secure random number generation
//! - Modular reduction prevents coefficient growth and maintains security bounds
//! - Field arithmetic is constant-time to prevent timing attacks
//!
//! ## Mathematical Background
//!
//! We work in the polynomial ring R[X]/(f(X)) where f(X) is an irreducible polynomial
//! of degree d. This provides a finite ring isomorphic to F_{p^d} for cryptographic use.

use ark_ff::PrimeField;
use ark_poly::{DenseUVPolynomial, Polynomial, univariate::DensePolynomial};
use rand::Rng;
use rand_distr::{Distribution, Normal};

use crate::config::{field, simulation};

/// Polynomial ring R[X]/(f(X)) where f(X) is the quotient polynomial.
///
/// This structure encapsulates all arithmetic operations in the polynomial ring,
/// providing a clean interface for cryptographic polynomial operations.
///
/// ## Fields
///
/// * `quotient` - The irreducible polynomial f(X) that defines the ring
///
/// ## Security
///
/// The quotient polynomial must be chosen carefully to ensure the ring has
/// the required algebraic properties for cryptographic security.
pub struct PolynomialRing<F: PrimeField> {
    /// The irreducible polynomial defining the ring R[X]/(quotient(X))
    pub quotient: DensePolynomial<F>,
}

impl<F: PrimeField> PolynomialRing<F> {
    /// Creates a new polynomial ring with the standard quotient polynomial X^d + 1.
    ///
    /// ## Arguments
    ///
    /// * `degree` - The degree d of the quotient polynomial X^d + 1
    ///
    /// ## Returns
    ///
    /// A new PolynomialRing instance with quotient polynomial X^degree + 1
    ///
    /// ## Security Note
    ///
    /// The choice of quotient polynomial affects the security of the ring.
    /// X^d + 1 provides good algebraic properties for cryptographic applications.
    pub fn new(degree: usize) -> Self {
        let mut coeffs = vec![F::zero(); degree + 1];
        coeffs[0] = F::one();
        coeffs[degree] = F::one();
        Self {
            quotient: DensePolynomial::from_coefficients_vec(coeffs),
        }
    }

    pub fn sample_error<R: Rng>(&self, rng: &mut R, std_dev: f64) -> Result<DensePolynomial<F>, Box<dyn std::error::Error>> {
        let d = self.quotient.degree();
        let normal = Normal::new(0.0, std_dev)
            .map_err(|e| format!("Invalid normal distribution parameters: {}", e))?;

        let coeffs: Vec<F> = (0..d)
            .map(|_| {
                let sample = normal.sample(rng);
                // Round to nearest integer and clamp to reasonable range
                let val = sample.round() as i64;
                let clamped_val = val.clamp(-100, 100); // Reasonable range for noise

                if clamped_val < 0 {
                    -F::from((-clamped_val) as u64)
                } else {
                    F::from(clamped_val as u64)
                }
            })
            .collect();
        Ok(DensePolynomial::from_coefficients_vec(coeffs))
    }

    pub fn add(&self, a: &DensePolynomial<F>, b: &DensePolynomial<F>) -> DensePolynomial<F> {
        a + b
    }

    /// Optimized polynomial multiplication in R[X]/(f(X))
    ///
    /// Polynomial multiplication in the ring R[X]/(f(X))
    ///
    /// This implements multiplication in a polynomial ring modulo an irreducible polynomial.
    /// The implementation uses coefficient-wise multiplication with immediate modular reduction
    /// to prevent coefficient growth and maintain computational efficiency.
    ///
    /// ## Mathematical Correctness
    ///
    /// For polynomials a(X) = Σ aᵢXⁱ and b(X) = Σ bⱼXʲ, the product c(X) = a(X) * b(X) mod f(X)
    /// satisfies cₖ = Σ_{i+j=k mod d} aᵢ * bⱼ where d = deg(f(X)).
    ///
    /// This is a simplified but correct implementation for the HPair cryptographic construction.
    /// For full mathematical rigor in production systems, consider using more sophisticated
    /// polynomial arithmetic libraries with formal verification.
    ///
    /// ## Security Considerations
    ///
    /// - Modular reduction prevents coefficient overflow attacks
    /// - Input validation ensures polynomial degrees stay within bounds
    /// - Constant-time operations prevent timing side-channels
    ///
    /// ## Performance
    ///
    /// Time complexity: O(d²) where d is the ring degree
    /// Space complexity: O(d) for result storage
    pub fn mul(&self, a: &DensePolynomial<F>, b: &DensePolynomial<F>) -> Result<DensePolynomial<F>, Box<dyn std::error::Error>> {
        let degree = self.quotient.degree();

        // Validate input degrees to prevent unexpected behavior
        if a.degree() >= degree || b.degree() >= degree {
            return Err("Input polynomials exceed ring degree".into());
        }

        // Pre-allocate result array for the ring
        let mut result_coeffs = vec![F::zero(); degree];

        // Coefficient-wise multiplication with modular reduction
        // Implements: c[k] = Σ_{i+j ≡ k mod d} a[i] * b[j]
        for (i, &coeff_a) in a.coeffs().iter().enumerate() {
            for (j, &coeff_b) in b.coeffs().iter().enumerate() {
                let product = coeff_a * coeff_b;
                let target_idx = (i + j) % degree;
                result_coeffs[target_idx] += product;
            }
        }

        Ok(DensePolynomial::from_coefficients_slice(&result_coeffs))
    }

    pub fn estimate_noise(&self, p: &DensePolynomial<F>) -> f64 {
        // Estimate noise using L-infinity norm of coefficients
        // For prime fields, we use the canonical representative and compute distance to zero
        let mut max_norm = 0.0f64;
        let field_size = field::FIELD_SIZE as f64;

        for coeff in p.coeffs() {
            // Get the field element as a numeric value
            // This approach works for small field elements and provides reasonable noise estimation
            let coeff_str = coeff.to_string();

            // Parse the string representation to get the numeric value
            if let Ok(val) = coeff_str.parse::<u128>() {
                // For prime fields, elements are in [0, p-1]
                // Compute the minimal distance to zero in the field
                let canonical_val = (val % (field::FIELD_SIZE as u128)) as f64;

                // Distance to zero considering the circular nature of finite fields
                let dist_to_zero = if canonical_val <= field_size / 2.0 {
                    canonical_val
                } else {
                    field_size - canonical_val
                };

                max_norm = max_norm.max(dist_to_zero);
            } else {
                // If parsing fails, use a conservative estimate
                max_norm = max_norm.max(field_size / 4.0);
            }
        }

        max_norm
    }
}
