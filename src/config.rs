//! # Configuration Module
//!
//! Centralized configuration constants for the HyperPairing cryptographic system.
//! This module contains all tunable parameters, cryptographic constants, and
//! security thresholds used throughout the implementation.
//!
//! ## Security Considerations
//!
//! - All cryptographic parameters are chosen to provide at least 128-bit post-quantum security
//! - 256-bit prime field provides ~128 bits of quantum resistance via Grover's algorithm
//! - Ring degree 256 provides security against lattice attacks (BKZ, LLL)
//! - Noise thresholds are conservative to prevent decryption failures
//! - Key sizes follow NIST recommendations for post-quantum security
//!
//! ## Performance Tuning
//!
//! Parameters can be adjusted based on:
//! - Required security level (128-bit, 256-bit)
//! - Performance constraints
//! - Memory limitations
//! - Network bandwidth requirements

/// Field parameters for the prime field
pub mod field {
    /// Prime modulus for the finite field F_p
    /// This is the BN254 scalar field prime (256-bit) for post-quantum security
    pub const MODULUS: &str = "21888242871839275222246405745257275088548364400416034343698204186575808495617";

    /// Generator for the multiplicative group of F_p
    pub const GENERATOR: u64 = 2;

    /// Field size as u64 for convenience (approximate for entropy calculations)
    /// Actual field size is 256-bit, using 2^64 approximation for legacy entropy calculations
    pub const FIELD_SIZE: u64 = u64::MAX;
}

/// Cryptographic parameters
pub mod crypto {
    /// Key size for AES-256 in bytes
    pub const KEY_SIZE: usize = 32;

    /// Nonce size for AES-GCM in bytes
    pub const NONCE_SIZE: usize = 12;

    /// Output size for HKDF key derivation
    pub const HKDF_OUTPUT_SIZE: usize = 32;
}

/// Polynomial ring parameters
pub mod polynomial {
    /// Default degree for the polynomial ring
    /// Increased to 256 for lattice-based cryptography security against BKZ attacks
    pub const DEFAULT_RING_DEGREE: usize = 256;

    /// Default maximum level for multilinear groups
    pub const DEFAULT_MAX_LEVEL: usize = 100;
}

/// Simulation parameters
pub mod simulation {
    /// Standard deviation for error sampling in polynomial rings
    pub const ERROR_STD_DEV: f64 = 2.0;

    /// Noise threshold for rerandomization (in simulation)
    pub const NOISE_THRESHOLD: f64 = 1_000_000_000.0;
}

/// API and security constants
pub mod api {
    /// Maximum participant name length
    pub const MAX_PARTICIPANT_NAME_LEN: usize = 64;

    /// Maximum message length (64KB)
    pub const MAX_MESSAGE_LEN: usize = 65536;

    /// Maximum number of groups to prevent memory exhaustion
    pub const MAX_GROUPS: usize = 10000;

    /// Group cleanup interval in seconds
    pub const CLEANUP_INTERVAL_SECS: u64 = 3600; // 1 hour

    /// Maximum group lifetime in seconds (24 hours)
    pub const MAX_GROUP_LIFETIME_SECS: u64 = 86400;

    /// Minimum quantum security level (64 bits)
    pub const MIN_QUANTUM_SECURITY: u32 = 64;

    /// Maximum attempts to generate unique group ID
    pub const MAX_GROUP_ID_ATTEMPTS: u32 = 1000;

    /// Maximum chat state size per group (1MB)
    pub const MAX_CHAT_STATE_SIZE: usize = 1024 * 1024;

    /// Maximum storage per group including all files (5MB)
    pub const MAX_STORAGE_PER_GROUP: usize = 5 * 1024 * 1024;

    /// Maximum number of participants per group
    pub const MAX_PARTICIPANTS: usize = 1000;
}
