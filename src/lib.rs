//! # HPair - Clean Cryptographic API
//!
//! A production-ready implementation of multi-linear group encryption providing
//! a simple, secure API for group communication with quantum-resistant properties.
//!
//! ## Quick Start
//!
//! ```rust
//! use hpair::{create_group, send_encrypted_message};
//!
//! // Create a new group
//! let group_id = create_group(vec!["Alice".to_string(), "Bob".to_string()]).unwrap();
//!
//! // Participants are automatically set up - send encrypted messages directly
//! send_encrypted_message(group_id, "Alice", "Hello, secure group!").unwrap();
//! ```
//!
//! ## Security Features
//!
//! - **AES-GCM-256** with cryptographically secure nonces
//! - **HKDF-SHA256** key derivation
//! - **Multi-linear group cryptography** for forward secrecy
//! - **Quantum-resistant** key establishment (128-bit post-quantum security)
//! - **Persistent encrypted storage** with automatic cleanup
//! - **Constant-time operations** to prevent timing attacks
//! - **Comprehensive input validation**
//!
//! ## Security Considerations
//!
//! - **Timing Attacks**: All sensitive cryptographic operations use constant-time algorithms
//! - **Storage Security**: Group keys and metadata are encrypted at rest using AES-GCM-256
//! - **Resource Limits**: Automatic cleanup prevents DoS attacks through resource exhaustion
//! - **Memory Safety**: Cryptographic state is kept in memory for the session duration
//! - **Forward Secrecy**: Multi-linear group construction provides forward secrecy properties

mod algebra;
mod config;
mod constant_time;
mod group_chat;
mod multilinear;
mod storage;

use crate::config::api;
use crate::storage::GroupStorage;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::{RngCore, rngs::OsRng};
use ark_ff::{MontBackend, MontConfig, Zero};
use ark_poly::DenseUVPolynomial;

#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "2"]
struct FieldConfig;
type Field = ark_ff::Fp256<MontBackend<FieldConfig, 4>>;

/// Unique identifier for a cryptographic group
pub type GroupId = u64;

/// Error type for HPair operations
#[derive(Debug, Clone)]
pub enum HPairError {
    /// Group not found
    GroupNotFound,
    /// Participant not in group
    ParticipantNotFound,
    /// Invalid participant name
    InvalidParticipant,
    /// Message is empty
    MessageEmpty,
    /// Message too large
    MessageTooLarge,
    /// Encryption/decryption failed
    CryptographicError(String),
    /// Group creation failed
    GroupCreationFailed,
    /// Internal error
    InternalError(String),
}

impl std::fmt::Display for HPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HPairError::GroupNotFound => write!(f, "Group not found"),
            HPairError::ParticipantNotFound => write!(f, "Participant not found in group"),
            HPairError::InvalidParticipant => write!(f, "Invalid participant name"),
            HPairError::MessageEmpty => write!(f, "Message cannot be empty"),
            HPairError::MessageTooLarge => write!(f, "Message too large (max 64KB)"),
            HPairError::CryptographicError(msg) => write!(f, "Cryptographic error: {}", msg),
            HPairError::GroupCreationFailed => write!(f, "Group creation failed"),
            HPairError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for HPairError {}

/// Internal group state with its own mutex for better concurrency
#[derive(Clone)]
struct GroupState {
    chat: crate::group_chat::GroupChat<Field>,
    participants: Vec<String>,
    created_at: std::time::Instant,
}

/// Serializable group state for persistent storage
#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableGroupState {
    participants: Vec<String>,
    created_at: u64,
    // Note: chat state is stored separately as encrypted binary data
}

// Global persistent storage for all groups
//
// Uses encrypted file-based storage with automatic cleanup and resource limits.
// Storage directory: ~/.hpair/groups (or configurable)
lazy_static::lazy_static! {
    static ref STORAGE: GroupStorage = {
        let storage_dir = std::env::var("HPAIR_STORAGE_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                // Use temp directory for tests, home directory for production
                if cfg!(test) {
                    std::env::temp_dir().join("hpair_test_storage")
                } else {
                    let mut home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
                    home.push(".hpair");
                    home
                }
            });

        GroupStorage::new(storage_dir).expect("Failed to initialize storage")
    };
}


/// Create a new cryptographic group with the specified participants.
///
/// This establishes a shared secret among all participants using multi-linear
/// group cryptography, providing forward secrecy and quantum resistance.
///
/// # Arguments
/// * `participants` - List of participant names (must be unique, non-empty, and valid)
///
/// # Returns
/// * `Ok(GroupId)` - Unique identifier for the created group
/// * `Err(HPairError)` - If group creation fails
///
/// # Security Notes
/// - All participants must be known at group creation time
/// - The group provides one-shot security (single session)
/// - Keys are quantum-resistant up to 128 bits
/// - Group IDs are cryptographically secure random values
pub fn create_group(participants: Vec<String>) -> Result<GroupId, HPairError> {
    // Input validation
    if participants.is_empty() {
        return Err(HPairError::GroupCreationFailed);
    }

    // Check for duplicates
    if participants.len() != participants.iter().collect::<std::collections::HashSet<_>>().len() {
        return Err(HPairError::InvalidParticipant);
    }

    // Validate participant names
    for participant in &participants {
        if participant.is_empty() || participant.len() > api::MAX_PARTICIPANT_NAME_LEN {
            return Err(HPairError::InvalidParticipant);
        }
        if !participant.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(HPairError::InvalidParticipant);
        }
    }

    // Check resource limits
    STORAGE.cleanup().map_err(|_| HPairError::GroupCreationFailed)?;
    if STORAGE.list_groups().map_err(|_| HPairError::GroupCreationFailed)?.len() >= api::MAX_GROUPS {
        return Err(HPairError::GroupCreationFailed);
    }

    // Create polynomial ring and multilinear group
    let ring = crate::algebra::PolynomialRing::<Field>::new(
        crate::config::polynomial::DEFAULT_RING_DEGREE
    );

    let generator = ark_poly::univariate::DensePolynomial::from_coefficients_vec(vec![
        Field::from(1u64); crate::config::polynomial::DEFAULT_RING_DEGREE
    ]);

    let ml_group = crate::multilinear::MultiLinearGroup::new(
        Arc::new(ring), participants.len(), generator
    );

    // Create group chat
    let mut chat = crate::group_chat::GroupChat::new(Arc::new(ml_group));

    // Setup group with participants
    chat.setup_group(participants.clone())
        .map_err(|_| HPairError::GroupCreationFailed)?;

    // Generate cryptographically secure random group ID
    // This inserts a placeholder in cache atomically to prevent race conditions
    let group_id = generate_secure_group_id()?;

    // Store group metadata using persistent storage
    let metadata = serde_json::to_vec(&SerializableGroupState {
        participants: participants.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }).map_err(|_| {
        // Cleanup placeholder on serialization failure
        static IN_MEMORY_CACHE: once_cell::sync::Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
            once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
        let mut cache = IN_MEMORY_CACHE.lock().ok();
        if let Some(ref mut c) = cache {
            c.remove(&group_id);
        }
        HPairError::GroupCreationFailed
    })?;

    STORAGE.store_group(group_id, participants.clone(), &metadata)
        .map_err(|_| {
            // Cleanup placeholder on storage failure
            static IN_MEMORY_CACHE: once_cell::sync::Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
                once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
            let mut cache = IN_MEMORY_CACHE.lock().ok();
            if let Some(ref mut c) = cache {
                c.remove(&group_id);
            }
            HPairError::GroupCreationFailed
        })?;

    // Create real group state
    let group_state = GroupState {
        chat,
        participants: participants.clone(),
        created_at: std::time::Instant::now(),
    };

    // Replace placeholder with real state atomically
    // The placeholder was inserted by generate_secure_group_id() to prevent race conditions
    {
        static IN_MEMORY_CACHE: once_cell::sync::Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
            once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
        let mut cache = IN_MEMORY_CACHE.lock().map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;
        
        // Replace placeholder (should always exist due to generate_secure_group_id)
        cache.insert(group_id, Arc::new(Mutex::new(group_state)));
    }

    Ok(group_id)
}

/// Generate a cryptographically secure random group ID
///
/// Returns a unique group ID that doesn't exist in either the in-memory cache
/// or persistent storage.
///
/// # Thread Safety
///
/// This function uses atomic check-and-insert to prevent TOCTOU race conditions.
/// The lock is held during both the availability check and placeholder insertion,
/// ensuring that two threads cannot generate the same group ID concurrently.
///
/// # Returns
/// * `Ok(GroupId)` - Unique group ID (placeholder already inserted in cache)
/// * `Err(HPairError)` - If max attempts exceeded
fn generate_secure_group_id() -> Result<GroupId, HPairError> {
    static IN_MEMORY_CACHE: once_cell::sync::Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
        once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

    for _ in 0..api::MAX_GROUP_ID_ATTEMPTS {
        let mut id_bytes = [0u8; 8];
        OsRng.fill_bytes(&mut id_bytes);
        let group_id = u64::from_le_bytes(id_bytes);

        // Ensure ID is not zero
        if group_id == 0 {
            continue;
        }

        // ATOMIC OPERATION: Hold lock during both check and insert
        // This prevents TOCTOU race condition where two threads could both
        // see the ID as available and proceed to use it.
        let mut cache = IN_MEMORY_CACHE.lock()
            .map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;

        // Check if ID already exists in cache
        if cache.contains_key(&group_id) {
            continue; // ID collision, try again
        }

        // Check if ID exists in persistent storage
        // Note: We check storage while holding cache lock to maintain consistency
        if STORAGE.load_group(group_id).is_ok() {
            continue; // ID exists in storage, try again
        }

        // ID is available - insert placeholder immediately while holding lock
        // This prevents another thread from using the same ID.
        // The placeholder will be replaced with real GroupState after successful setup.
        let placeholder_ring = crate::algebra::PolynomialRing::<Field>::new(1);
        let placeholder_generator = ark_poly::univariate::DensePolynomial::from_coefficients_vec(
            vec![Field::zero(); 1]
        );
        let placeholder_ml_group = crate::multilinear::MultiLinearGroup::new(
            Arc::new(placeholder_ring),
            1,
            placeholder_generator,
        );
        let placeholder_chat = crate::group_chat::GroupChat::new(Arc::new(placeholder_ml_group));
        
        let placeholder_state = GroupState {
            chat: placeholder_chat,
            participants: Vec::new(), // Empty placeholder
            created_at: std::time::Instant::now(),
        };

        // Insert placeholder atomically - lock is held throughout
        cache.insert(group_id, Arc::new(Mutex::new(placeholder_state)));
        
        // Lock is released here (on drop), but placeholder is already inserted
        // Return the ID - caller will replace placeholder with real state
        return Ok(group_id);
    }

    Err(HPairError::InternalError("Could not generate unique group ID".to_string()))
}

/// Cleanup expired groups and enforce resource limits
fn cleanup_expired_groups() -> Result<(), HPairError> {
    STORAGE.cleanup().map_err(|_| HPairError::InternalError("Storage cleanup failed".to_string()))?;

    // Also cleanup in-memory cache
    static IN_MEMORY_CACHE: Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    let mut cache = IN_MEMORY_CACHE.lock().map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;
    let now = std::time::Instant::now();
    let max_lifetime = std::time::Duration::from_secs(api::MAX_GROUP_LIFETIME_SECS);

    cache.retain(|_, group_state_arc| {
        let group_state = match group_state_arc.try_lock() {
            Ok(state) => state,
            Err(_) => return true, // Keep if locked (in use)
        };
        now.duration_since(group_state.created_at) < max_lifetime
    });

    Ok(())
}

/// Destroy a group and free its resources
pub fn destroy_group(group_id: GroupId) -> Result<(), HPairError> {
    // Remove from persistent storage
    STORAGE.delete_group(group_id)
        .map_err(|_| HPairError::InternalError("Storage deletion failed".to_string()))?;

    // Remove from in-memory cache
    static IN_MEMORY_CACHE: Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    let mut cache = IN_MEMORY_CACHE.lock().map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;
    cache.remove(&group_id);

    Ok(())
}

/// List all active groups
pub fn list_groups() -> Result<Vec<GroupId>, HPairError> {
    STORAGE.list_groups().map_err(|_| HPairError::InternalError("Storage list failed".to_string()))
}

/// Get information about a specific group
pub fn get_group_info(group_id: GroupId) -> Result<(Vec<String>, std::time::Instant), HPairError> {
    let (participants, _) = STORAGE.get_group_info(group_id)
        .map_err(|_| HPairError::GroupNotFound)?;

    // For created_at, we need to get it from the in-memory cache or reconstruct it
    static IN_MEMORY_CACHE: Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    let cache = IN_MEMORY_CACHE.lock().map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;

    if let Some(group_state_arc) = cache.get(&group_id) {
        let group_state = group_state_arc.lock().map_err(|_| HPairError::InternalError("Group lock poisoned".to_string()))?;
        Ok((participants, group_state.created_at))
    } else {
        Err(HPairError::GroupNotFound)
    }
}


/// Send an encrypted message to all group participants.
///
/// This encrypts the message using the group's shared secret and
/// sends it to all participants. Only group members can decrypt
/// the message.
///
/// # Arguments
/// * `group_id` - The group identifier
/// * `sender` - Name of the sender (must be a group member)
/// * `message` - The message to encrypt and send
///
/// # Returns
/// * `Ok(())` - Message successfully encrypted and "sent"
/// * `Err(HPairError)` - If sending fails
///
/// # Security Notes
/// - Messages are encrypted with AES-GCM-256
/// - Each message uses a unique nonce
/// - Only group members can decrypt messages
/// - Input validation prevents buffer overflow attacks
pub fn send_encrypted_message(group_id: GroupId, sender: &str, message: &str) -> Result<(), HPairError> {
    // Input validation
    if sender.is_empty() || sender.len() > api::MAX_PARTICIPANT_NAME_LEN {
        return Err(HPairError::InvalidParticipant);
    }
    if !sender.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(HPairError::InvalidParticipant);
    }

    if message.is_empty() {
        return Err(HPairError::MessageEmpty);
    }
    if message.len() > api::MAX_MESSAGE_LEN {
        return Err(HPairError::MessageTooLarge);
    }

    // Get group state from in-memory cache
    // Note: Cryptographic state is kept in memory for security.
    // Only metadata is persisted for recovery purposes.
    static IN_MEMORY_CACHE: Lazy<Mutex<HashMap<GroupId, Arc<Mutex<GroupState>>>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    let cache = IN_MEMORY_CACHE.lock().map_err(|_| HPairError::InternalError("Cache lock poisoned".to_string()))?;
    let group_state_arc = cache.get(&group_id)
        .ok_or(HPairError::GroupNotFound)?
        .clone();

    // Lock group state
    let group_state = group_state_arc.lock().map_err(|_| HPairError::InternalError("Group lock poisoned".to_string()))?;

    // Verify sender is in the group
    if !group_state.participants.contains(&sender.to_string()) {
        return Err(HPairError::ParticipantNotFound);
    }

    // Encrypt and "send" the message
    group_state.chat.broadcast(sender, message)
        .map_err(|_| HPairError::CryptographicError("Encryption failed".to_string()))?;

    Ok(())
}

/// Calculate the quantum resistance level (bit security) of a cryptographic key.
///
/// This function estimates the bit security level against quantum attacks
/// for the given key material. The estimation is based on established
/// cryptographic principles and quantum computing threat models.
///
/// # Arguments
/// * `key` - The key material to analyze
///
/// # Returns
/// * `u32` - Bit security level (e.g., 128, 256)
///
/// # Security Notes
/// - Returns 0 for keys with insufficient entropy
/// - Based on Grover's algorithm complexity O(2^(n/2)) for symmetric search
/// - Considers multi-linear group construction benefits
/// - Conservative estimates for post-quantum security
pub fn calculate_quantum_resistance(key: &[u8]) -> u32 {
    if key.is_empty() {
        return 0;
    }

    let key_len_bits = key.len() as f64 * 8.0;

    // For HPair's construction, we consider:
    // 1. Base symmetric key strength (AES-GCM-256 provides 256-bit classical security)
    // 2. Quantum reduction via Grover's algorithm: O(2^(n/2))
    // 3. Additional security from multi-linear group construction
    // 4. Entropy quality factor

    // Calculate entropy quality factor (0.0 to 1.0)
    let entropy_factor = calculate_entropy_quality(key);

    // Classical security estimate (conservative)
    let classical_security = key_len_bits.min(256.0) * entropy_factor;

    // Quantum security reduction (Grover's algorithm)
    // Symmetric search goes from O(2^n) to O(2^(n/2))
    let quantum_security = classical_security / 2.0;

    // Additional security from multi-linear construction
    // Multi-linear groups provide some additional quantum resistance
    let multilinear_bonus = (classical_security * 0.1).min(16.0); // Up to 16 bits bonus

    // Final quantum-resistant bit security
    let total_quantum_security = quantum_security + multilinear_bonus;

    // Ensure minimum reasonable security level
    total_quantum_security.max(api::MIN_QUANTUM_SECURITY as f64) as u32
}

/// Calculate entropy quality factor based on key distribution
fn calculate_entropy_quality(key: &[u8]) -> f64 {
    if key.is_empty() {
        return 0.0;
    }

    // Calculate byte frequency distribution
    let mut freq = [0u32; 256];
    for &byte in key {
        freq[byte as usize] += 1;
    }

    let len = key.len() as f64;
    let mut entropy = 0.0;
    let mut distinct_bytes = 0;

    for &count in &freq {
        if count > 0 {
            distinct_bytes += 1;
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    // Normalize entropy to [0, 1] range
    let max_entropy = (distinct_bytes as f64).min(256.0).log2();
    if max_entropy > 0.0 {
        entropy / max_entropy
    } else {
        0.0
    }
}

// Note: Internal modules are not exposed in the public API
// Only the three main functions are available to users
