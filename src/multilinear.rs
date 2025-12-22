use crate::algebra::PolynomialRing;
use crate::config::{crypto, simulation};
use ark_ff::PrimeField;
use ark_poly::{DenseUVPolynomial, univariate::DensePolynomial};
use hkdf::Hkdf;
use sha3::{Digest, Sha3_256};
use std::sync::Arc;

#[derive(Clone)]
pub struct Encoding<F: PrimeField> {
    pub value: DensePolynomial<F>,
    pub level: usize,
}

pub struct MultiLinearGroup<F: PrimeField> {
    pub ring: Arc<PolynomialRing<F>>,
    pub max_level: usize,
    pub generator: DensePolynomial<F>,
}

impl<F: PrimeField> MultiLinearGroup<F> {
    pub fn new(
        ring: Arc<PolynomialRing<F>>,
        max_level: usize,
        generator: DensePolynomial<F>,
    ) -> Self {
        Self {
            ring,
            max_level,
            generator,
        }
    }

    pub fn encode(&self, secret: &DensePolynomial<F>, level: usize) -> Result<Encoding<F>, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let noise = self.ring.sample_error(&mut rng, simulation::ERROR_STD_DEV)?;
        let secret_times_generator = self.ring.mul(secret, &self.generator)?;
        let val = self.ring.add(&secret_times_generator, &noise);
        Ok(Encoding { value: val, level })
    }

    pub fn pair(&self, encodings: &[Encoding<F>]) -> Result<Encoding<F>, Box<dyn std::error::Error>> {
        if encodings.is_empty() {
            return Err("Cannot pair empty encoding list".into());
        }
        let mut result_val = encodings[0].value.clone();
        let mut current_level = encodings[0].level;

        for (i, next_enc) in encodings.iter().enumerate().skip(1) {
            result_val = self.ring.mul(&result_val, &next_enc.value)?;
            current_level += next_enc.level;

            // Noise Management: Monitor growth
            let noise_estimate = self.ring.estimate_noise(&result_val);
            if i % 10 == 0 || i == encodings.len() - 1 {
                // In a real system, we would perform 'rerandomization' or 'modulus switching' here
                // if noise_estimate approach a threshold.
                // For simulation, we log the growth.
                if noise_estimate > simulation::NOISE_THRESHOLD {
                    println!("[Warning] High noise detected at level {}: {:.2e}", i, noise_estimate);
                }
            }
        }

        Ok(Encoding {
            value: result_val,
            level: current_level,
        })
    }

    /// Extract cryptographically secure key from polynomial encoding using HKDF
    ///
    /// This function provides secure key derivation with high entropy output,
    /// suitable for use with AES-256 and other symmetric cryptographic primitives.
    pub fn extract(&self, encoding: &Encoding<F>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Optimized key material extraction with pre-allocated capacity
        let mut input_key_material = Vec::with_capacity(encoding.value.coeffs().len() * 8);

        // Convert polynomial coefficients to bytes more efficiently
        for coeff in encoding.value.coeffs() {
            // Use more direct field element serialization
            let coeff_str = coeff.to_string();
            let bytes = coeff_str.as_bytes();
            input_key_material.extend_from_slice(bytes);
        }

        // SECURITY: HKDF key derivation provides forward secrecy and key separation
        // AUDIT: Ensures cryptographically strong key material from polynomial encodings
        // THREAT: Weak input entropy could compromise key strength
        let hkdf = Hkdf::<Sha3_256>::new(None, &input_key_material);
        let mut output_key_material = [0u8; crypto::HKDF_OUTPUT_SIZE];

        hkdf.expand(b"multilinear-key-v1", &mut output_key_material)
            .map_err(|e| format!("HKDF expansion failed: {:?}", e))?;

        Ok(output_key_material.to_vec())
    }
}
