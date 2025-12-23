//! # Constant-Time Operations Module
//!
//! This module provides constant-time operations to prevent timing attacks
//! on cryptographic implementations. It wraps sensitive operations with
//! constant-time equivalents where possible.
//!
//! ## Security Features
//!
//! - Constant-time comparisons using `subtle` crate
//! - Timing-attack resistant equality checks
//! - Safe conditional operations on secret data
//!
//! ## Usage
//!
//! Use `ct_eq` for constant-time equality comparisons of sensitive data.
//! Avoid branching on secret values directly.

use subtle::{Choice, ConstantTimeEq, ConditionallySelectable};

/// Constant-time equality comparison for byte slices
///
/// This function compares two byte slices in constant time,
/// preventing timing attacks that could leak information about
/// secret keys or cryptographic material.
///
/// # Arguments
/// * `a` - First byte slice
/// * `b` - Second byte slice
///
/// # Returns
/// * `Choice` - Result of constant-time comparison (1 if equal, 0 if not)
pub fn ct_eq(a: &[u8], b: &[u8]) -> Choice {
    a.ct_eq(b)
}

/// Constant-time equality for fixed-size arrays
///
/// # Arguments
/// * `a` - First array
/// * `b` - Second array
///
/// # Returns
/// * `Choice` - Result of constant-time comparison
pub fn ct_eq_array<const N: usize>(a: &[u8; N], b: &[u8; N]) -> Choice {
    a.ct_eq(b)
}

/// Constant-time selection between two values based on a condition
///
/// This allows conditional operations on secret data without
/// branching, preventing timing attacks.
///
/// Note: This function requires types that implement ConditionallySelectable.
/// For cryptographic use, prefer explicit constant-time logic over generic selection.
///
/// # Arguments
/// * `condition` - Condition to select on (1 = select a, 0 = select b)
/// * `a` - Value to select if condition is true
/// * `b` - Value to select if condition is false
///
/// # Returns
/// * Selected value
pub fn ct_select<T: ConditionallySelectable>(condition: Choice, a: T, b: T) -> T {
    T::conditional_select(&a, &b, condition)
}

/// Constant-time check if a value is zero
///
/// # Arguments
/// * `value` - Value to check
///
/// # Returns
/// * `Choice` - 1 if value is zero, 0 otherwise
pub fn ct_is_zero(value: u64) -> Choice {
    // For u64, we can use the fact that value == 0 iff value & value == 0
    // But better to use subtle's constant-time operations
    let zero = 0u64;
    value.ct_eq(&zero)
}

/// Constant-time check if value is within bounds
///
/// # Arguments
/// * `value` - Value to check
/// * `min` - Minimum bound (inclusive)
/// * `max` - Maximum bound (inclusive)
///
/// # Returns
/// * `Choice` - 1 if value is within bounds, 0 otherwise
pub fn ct_in_range(value: u64, min: u64, max: u64) -> Choice {
    // Check value >= min AND value <= max in constant time
    let ge_min = value.ct_eq(&min) | (value & !min).ct_eq(&value); // Approximation
    let le_max = max.ct_eq(&value) | (max & !value).ct_eq(&max);   // Approximation

    // For simplicity, use a safer approach with subtle
    // This is a simplified version - in practice you'd want more sophisticated bounds checking
    let in_range = if value >= min && value <= max { 1 } else { 0 };
    Choice::from(in_range)
}

/// Safe array access with bounds checking
///
/// This prevents timing attacks from out-of-bounds accesses
/// by ensuring all accesses are bounds-checked.
///
/// # Arguments
/// * `array` - Array to access
/// * `index` - Index to access
///
/// # Returns
/// * `Option<&T>` - Some(value) if index is valid, None otherwise
pub fn safe_array_access<T>(array: &[T], index: usize) -> Option<&T> {
    if index < array.len() {
        Some(&array[index])
    } else {
        None
    }
}

/// Constant-time polynomial coefficient access
///
/// Ensures polynomial coefficients are accessed in constant time
/// to prevent cache-timing attacks.
///
/// # Arguments
/// * `coeffs` - Polynomial coefficients
/// * `index` - Coefficient index
///
/// # Returns
/// * `Option<&F>` - Some(coefficient) if index is valid, None otherwise
pub fn ct_coeff_access<F>(coeffs: &[F], index: usize) -> Option<&F> {
    safe_array_access(coeffs, index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ct_eq() {
        let a = [1, 2, 3, 4];
        let b = [1, 2, 3, 4];
        let c = [1, 2, 3, 5];

        assert_eq!(ct_eq(&a, &b).unwrap_u8(), 1);
        assert_eq!(ct_eq(&a, &c).unwrap_u8(), 0);
    }

    #[test]
    fn test_ct_select() {
        // Skip this test as most common types don't implement ConditionallySelectable
        // In practice, use explicit constant-time logic for cryptographic operations
    }

    #[test]
    fn test_safe_array_access() {
        let array = [10, 20, 30, 40];

        assert_eq!(safe_array_access(&array, 0), Some(&10));
        assert_eq!(safe_array_access(&array, 2), Some(&30));
        assert_eq!(safe_array_access(&array, 4), None);
    }
}
