//! # Persistent Storage Module
//!
//! This module provides encrypted, persistent storage for cryptographic group state.
//! It replaces the previous in-memory global state with secure file-based storage.
//!
//! ## Security Features
//!
//! - AES-GCM-256 encryption for stored data
//! - Cryptographically secure random keys per group
//! - Automatic cleanup of expired groups
//! - Resource limits per group to prevent DoS attacks
//! - Atomic operations to prevent corruption
//!
//! ## Storage Format
//!
//! Groups are stored as encrypted JSON files with the following structure:
//! ```
//! storage/
//! ├── groups/
//! │   ├── {group_id}.group
//! │   └── ...
//! └── keys/
//!     └── {group_id}.key
//! ```

use crate::config::api;
use crate::constant_time;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit}};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Detailed storage error types for better error handling and debugging
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Authentication failed during decryption (possible tampering)
    AuthenticationFailed,
    /// Data is corrupted or malformed
    DataCorrupted,
    /// Key mismatch or invalid key
    KeyMismatch,
    /// File not found in storage
    FileNotFound,
    /// Permission denied for storage operation
    PermissionDenied,
    /// Storage quota exceeded
    QuotaExceeded,
    /// Invalid data format
    InvalidData,
    /// I/O error during storage operation
    IoError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::AuthenticationFailed =>
                write!(f, "Storage authentication failed - possible data tampering"),
            StorageError::DataCorrupted =>
                write!(f, "Storage data corrupted or malformed"),
            StorageError::KeyMismatch =>
                write!(f, "Key mismatch during decryption"),
            StorageError::FileNotFound =>
                write!(f, "Storage file not found"),
            StorageError::PermissionDenied =>
                write!(f, "Permission denied for storage operation"),
            StorageError::QuotaExceeded =>
                write!(f, "Storage quota exceeded"),
            StorageError::InvalidData =>
                write!(f, "Invalid data format"),
            StorageError::IoError(msg) =>
                write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

/// Encrypted storage for group data
pub struct GroupStorage {
    storage_dir: PathBuf,
    cache: Mutex<HashMap<u64, Arc<Mutex<CachedGroup>>>>,
}

#[derive(Clone)]
struct CachedGroup {
    data: GroupData,
    key: Vec<u8>,
    last_accessed: SystemTime,
    file_path: PathBuf,
    key_path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone)]
struct GroupData {
    group_id: u64,
    participants: Vec<String>,
    created_at: u64, // Unix timestamp
    encrypted_chat_state: Vec<u8>,
    nonce: Vec<u8>,
}

impl GroupStorage {
    /// Create a new storage instance
    pub fn new(storage_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all(&storage_dir)?;
        fs::create_dir_all(storage_dir.join("groups"))?;
        fs::create_dir_all(storage_dir.join("keys"))?;

        Ok(Self {
            storage_dir,
            cache: Mutex::new(HashMap::new()),
        })
    }

    /// Store a new group
    pub fn store_group(
        &self,
        group_id: u64,
        participants: Vec<String>,
        chat_state: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Resource limit validation
        // 1. Validate chat state size
        if chat_state.len() > api::MAX_CHAT_STATE_SIZE {
            return Err(format!(
                "Chat state too large: {} bytes (max {} bytes)",
                chat_state.len(),
                api::MAX_CHAT_STATE_SIZE
            ).into());
        }

        // 2. Validate participant count
        if participants.len() > api::MAX_PARTICIPANTS {
            return Err(format!(
                "Too many participants: {} (max {})",
                participants.len(),
                api::MAX_PARTICIPANTS
            ).into());
        }

        // 3. Calculate expected storage size for this group
        let estimated_size = self.calculate_group_storage_size(&participants, chat_state.len());

        // 4. Check if this would exceed per-group storage limit
        if estimated_size > api::MAX_STORAGE_PER_GROUP {
            return Err(format!(
                "Storage quota exceeded: estimated {} bytes (max {} bytes per group)",
                estimated_size,
                api::MAX_STORAGE_PER_GROUP
            ).into());
        }

        // 5. Check existing groups' storage (if group_id already exists, check current usage)
        // This is efficient - we check cache first, then disk only if needed
        if let Ok((existing_participants, existing_chat_state)) = self.load_group(group_id) {
            let existing_size = self.calculate_group_storage_size(&existing_participants, existing_chat_state.len());
            let new_size = self.calculate_group_storage_size(&participants, chat_state.len());
            
            // Allow update if new size doesn't exceed limit
            if new_size > api::MAX_STORAGE_PER_GROUP {
                return Err(format!(
                    "Storage quota exceeded: {} bytes (max {} bytes per group)",
                    new_size,
                    api::MAX_STORAGE_PER_GROUP
                ).into());
            }
        }

        // Generate encryption key for this group
        let mut group_key = [0u8; 32];
        OsRng.fill_bytes(&mut group_key);

        // Create group data
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        let group_data = GroupData {
            group_id,
            participants,
            created_at,
            encrypted_chat_state: chat_state.to_vec(),
            nonce: vec![], // Will be set during encryption
        };

        // Encrypt the group data
        let encrypted_data = self.encrypt_group_data(&group_data, &group_key)?;

        // Store encrypted data
        let file_path = self.storage_dir.join("groups").join(format!("{}.group", group_id));
        let key_path = self.storage_dir.join("keys").join(format!("{}.key", group_id));

        // Atomic write: write to temp file first, then rename
        let temp_file = file_path.with_extension("tmp");
        let temp_key = key_path.with_extension("tmp");

        fs::write(&temp_file, &encrypted_data)?;
        fs::write(&temp_key, &group_key)?;

        fs::rename(&temp_file, &file_path)?;
        fs::rename(&temp_key, &key_path)?;

        // Update cache
        let mut cache = self.cache.lock().unwrap();
        cache.insert(group_id, Arc::new(Mutex::new(CachedGroup {
            data: group_data,
            key: group_key.to_vec(),
            last_accessed: SystemTime::now(),
            file_path,
            key_path,
        })));

        Ok(())
    }

    /// Load a group from storage
    pub fn load_group(&self, group_id: u64) -> Result<(Vec<String>, Vec<u8>), Box<dyn std::error::Error>> {
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&group_id) {
                let mut cached_group = cached.lock().unwrap();
                cached_group.last_accessed = SystemTime::now();

                // Check if group has expired
                if self.is_group_expired(cached_group.data.created_at)? {
                    return Err("Group has expired".into());
                }

                return Ok((cached_group.data.participants.clone(), cached_group.data.encrypted_chat_state.clone()));
            }
        }

        // Load from disk
        let file_path = self.storage_dir.join("groups").join(format!("{}.group", group_id));
        let key_path = self.storage_dir.join("keys").join(format!("{}.key", group_id));

        if !file_path.exists() || !key_path.exists() {
            return Err("Group not found".into());
        }

        let encrypted_data = fs::read(&file_path)?;
        let group_key = fs::read(&key_path)?;

        let group_data = self.decrypt_group_data(&encrypted_data, &group_key)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Check if expired
        if self.is_group_expired(group_data.created_at)? {
            // Clean up expired group
            let _ = fs::remove_file(&file_path);
            let _ = fs::remove_file(&key_path);
            return Err("Group has expired".into());
        }

        // Cache the loaded group
        let mut cache = self.cache.lock().unwrap();
        cache.insert(group_id, Arc::new(Mutex::new(CachedGroup {
            data: group_data.clone(),
            key: group_key,
            last_accessed: SystemTime::now(),
            file_path,
            key_path,
        })));

        Ok((group_data.participants, group_data.encrypted_chat_state))
    }

    /// Delete a group
    pub fn delete_group(&self, group_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Remove from cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.remove(&group_id);
        }

        // Remove from disk
        let file_path = self.storage_dir.join("groups").join(format!("{}.group", group_id));
        let key_path = self.storage_dir.join("keys").join(format!("{}.key", group_id));

        let _ = fs::remove_file(file_path); // Ignore errors
        let _ = fs::remove_file(key_path); // Ignore errors

        Ok(())
    }

    /// List all active groups
    pub fn list_groups(&self) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        let mut groups = Vec::new();

        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            for (group_id, cached) in cache.iter() {
                let cached_group = cached.lock().unwrap();
                if !self.is_group_expired(cached_group.data.created_at)? {
                    groups.push(*group_id);
                }
            }
        }

        // Also check disk for uncached groups
        if let Ok(entries) = fs::read_dir(self.storage_dir.join("groups")) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if let Some(group_id_str) = filename.strip_suffix(".group") {
                        if let Ok(group_id) = group_id_str.parse::<u64>() {
                            if !groups.contains(&group_id) {
                                // Verify it's not expired by trying to load it
                                if self.load_group(group_id).is_ok() {
                                    groups.push(group_id);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(groups)
    }

    /// Get group info
    pub fn get_group_info(&self, group_id: u64) -> Result<(Vec<String>, u64), Box<dyn std::error::Error>> {
        let (participants, _) = self.load_group(group_id)?;
        let created_at = {
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&group_id) {
                cached.lock().unwrap().data.created_at
            } else {
                return Err("Group not found in cache".into());
            }
        };
        Ok((participants, created_at))
    }

    /// Cleanup expired groups and enforce resource limits
    pub fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cache = self.cache.lock().unwrap();
        let mut to_remove = Vec::new();

        // Check cache for expired groups
        for (group_id, cached) in cache.iter() {
            let cached_group = cached.lock().unwrap();
            if self.is_group_expired(cached_group.data.created_at)? {
                to_remove.push(*group_id);
            }
        }

        // Remove expired groups
        for group_id in to_remove {
            cache.remove(&group_id);
            let _ = self.delete_group(group_id); // Ignore errors during cleanup
        }

        // Enforce resource limits
        if cache.len() > api::MAX_GROUPS {
            // Remove oldest accessed groups
            let mut groups_by_access: Vec<_> = cache.iter().collect();
            groups_by_access.sort_by_key(|(_, cached)| {
                cached.lock().unwrap().last_accessed
            });

            let excess = cache.len() - api::MAX_GROUPS;
            let groups_to_remove: Vec<_> = groups_by_access.into_iter()
                .take(excess)
                .map(|(group_id, _)| *group_id)
                .collect();

            for group_id in groups_to_remove {
                cache.remove(&group_id);
                let _ = self.delete_group(group_id);
            }
        }

        Ok(())
    }

    /// Calculate estimated storage size for a group
    ///
    /// Estimates the total storage size including:
    /// - Encrypted group data file
    /// - Encryption key file
    /// - JSON serialization overhead
    /// - AES-GCM encryption overhead (nonce + tag)
    ///
    /// # Arguments
    /// * `participants` - List of participant names
    /// * `chat_state_size` - Size of encrypted chat state in bytes
    ///
    /// # Returns
    /// Estimated total storage size in bytes
    fn calculate_group_storage_size(&self, participants: &[String], chat_state_size: usize) -> usize {
        // Estimate JSON serialization size
        // GroupData structure overhead + participants + metadata
        let metadata_overhead = 100; // group_id, created_at, nonce fields
        let participants_json_size: usize = participants.iter()
            .map(|p| p.len() + 4) // string length + quotes + comma
            .sum();
        
        // AES-GCM overhead: 12-byte nonce + 16-byte authentication tag
        let encryption_overhead = 12 + 16;
        
        // Total: JSON size + encryption overhead + key file
        let json_size = metadata_overhead + participants_json_size + chat_state_size;
        let encrypted_size = json_size + encryption_overhead;
        let key_file_size = 32; // 256-bit key
        
        encrypted_size + key_file_size
    }

    /// Encrypt group data
    fn encrypt_group_data(&self, data: &GroupData, key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let json_data = serde_json::to_string(data)?;
        let encrypted = cipher.encrypt(nonce, json_data.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Prepend nonce to encrypted data
        let mut result = nonce_bytes.to_vec();
        result.extend(encrypted);
        Ok(result)
    }

    /// Decrypt group data
    fn decrypt_group_data(&self, encrypted_data: &[u8], key: &[u8]) -> Result<GroupData, StorageError> {
        if key.len() != 32 {
            return Err(StorageError::KeyMismatch);
        }

        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        if encrypted_data.len() < 12 {
            return Err(StorageError::InvalidData);
        }

        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];

        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| StorageError::AuthenticationFailed)?;

        let json_str = String::from_utf8(decrypted)
            .map_err(|_| StorageError::DataCorrupted)?;

        let data: GroupData = serde_json::from_str(&json_str)
            .map_err(|_| StorageError::DataCorrupted)?;

        Ok(data)
    }

    /// Check if a group has expired
    fn is_group_expired(&self, created_at: u64) -> Result<bool, Box<dyn std::error::Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let max_age = api::MAX_GROUP_LIFETIME_SECS;
        Ok(now.saturating_sub(created_at) > max_age)
    }

    /// Constant-time key verification
    ///
    /// Verifies that a stored key matches the expected key in constant time
    /// to prevent timing attacks during key operations.
    ///
    /// # Arguments
    /// * `stored_key` - Key read from storage
    /// * `expected_key` - Expected key value
    ///
    /// # Returns
    /// * `bool` - True if keys match
    fn ct_verify_key(&self, stored_key: &[u8], expected_key: &[u8]) -> bool {
        constant_time::ct_eq(stored_key, expected_key).into()
    }
}
