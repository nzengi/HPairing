//! # Group Chat Module
//!
//! This module provides secure group messaging using a ratchet tree structure
//! for key exchange. Messages are encrypted using AES-GCM-256 with the group
//! secret derived from the tree.
//!
//! ## Security Features
//!
//! - **Tree-based DH**: MLS-style ratchet tree for group key agreement
//! - **Forward Secrecy**: Keys change when members are removed
//! - **AES-GCM-256**: Authenticated encryption for messages
//! - **Unique Nonces**: Cryptographically secure random nonces per message

use crate::ratchet_tree::{RatchetTree, TreeError};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;

/// Error types for group chat operations
#[derive(Debug, Clone)]
pub enum GroupChatError {
    /// Group has not been set up
    NotInitialized,
    /// Tree operation failed
    TreeError(String),
    /// Encryption/decryption failed
    CryptoError(String),
    /// Invalid input
    InvalidInput(String),
}

impl std::fmt::Display for GroupChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupChatError::NotInitialized => write!(f, "Group not initialized"),
            GroupChatError::TreeError(msg) => write!(f, "Tree error: {}", msg),
            GroupChatError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            GroupChatError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for GroupChatError {}

impl From<TreeError> for GroupChatError {
    fn from(e: TreeError) -> Self {
        GroupChatError::TreeError(e.to_string())
    }
}

/// Group chat with tree-based key exchange
#[derive(Clone)]
pub struct GroupChat {
    /// The ratchet tree for key agreement
    tree: RatchetTree,
    /// Cached group secret (derived from tree root)
    shared_secret: Option<[u8; 32]>,
}

impl GroupChat {
    /// Create a new empty group chat
    pub fn new() -> Self {
        Self {
            tree: RatchetTree::empty(),
            shared_secret: None,
        }
    }

    /// Set up a group with initial participants
    pub fn setup_group(&mut self, participant_names: Vec<String>) -> Result<(), GroupChatError> {
        if participant_names.is_empty() {
            return Err(GroupChatError::InvalidInput(
                "No participants provided".to_string(),
            ));
        }

        // Validate participant names
        for name in &participant_names {
            if name.is_empty() || name.len() > 64 {
                return Err(GroupChatError::InvalidInput(format!(
                    "Invalid participant name: {}",
                    name
                )));
            }
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(GroupChatError::InvalidInput(format!(
                    "Invalid characters in name: {}",
                    name
                )));
            }
        }

        println!(
            "[Setup] Initializing Tree-DH Group for: {}",
            participant_names.join(", ")
        );

        // Create the ratchet tree
        self.tree = RatchetTree::new(participant_names);

        // Derive and cache the group secret
        self.shared_secret = Some(self.tree.derive_group_secret()?);

        println!(
            "[Setup] Group secret established for {} participants using Tree-DH.",
            self.tree.member_count()
        );

        Ok(())
    }

    /// Get the shared secret (for persistence)
    pub fn get_shared_secret(&self) -> Option<Vec<u8>> {
        self.shared_secret.map(|s| s.to_vec())
    }

    /// Set the shared secret (when restoring from persistence)
    pub fn set_shared_secret(&mut self, secret: Option<Vec<u8>>) {
        self.shared_secret = secret.and_then(|s| {
            if s.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&s);
                Some(arr)
            } else {
                None
            }
        });
    }

    /// Get the list of participants
    pub fn participants(&self) -> Vec<String> {
        self.tree.members()
    }

    /// Check if a participant is in the group
    pub fn has_participant(&self, name: &str) -> bool {
        self.tree.members().iter().any(|n| n == name)
    }

    /// Add a new member to the group
    pub fn add_member(&mut self, name: String) -> Result<(), GroupChatError> {
        if name.is_empty() || name.len() > 64 {
            return Err(GroupChatError::InvalidInput(format!(
                "Invalid participant name: {}",
                name
            )));
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(GroupChatError::InvalidInput(format!(
                "Invalid characters in name: {}",
                name
            )));
        }

        println!("[Group] Adding member: {}", name);

        self.tree.add_member(name)?;
        self.shared_secret = Some(self.tree.derive_group_secret()?);

        println!(
            "[Group] Member added. New group size: {}",
            self.tree.member_count()
        );
        Ok(())
    }

    /// Remove a member from the group
    pub fn remove_member(&mut self, name: &str) -> Result<(), GroupChatError> {
        println!("[Group] Removing member: {}", name);

        self.tree.remove_member(name)?;
        self.shared_secret = Some(self.tree.derive_group_secret()?);

        println!(
            "[Group] Member removed. New group size: {}. Keys refreshed for forward secrecy.",
            self.tree.member_count()
        );
        Ok(())
    }

    /// Broadcast an encrypted message to the group
    pub fn broadcast(
        &self,
        sender: &str,
        message: &str,
    ) -> Result<(Vec<u8>, Vec<u8>), GroupChatError> {
        // Input validation
        if sender.is_empty() {
            return Err(GroupChatError::InvalidInput(
                "Sender name cannot be empty".to_string(),
            ));
        }
        if message.is_empty() {
            return Err(GroupChatError::InvalidInput(
                "Message cannot be empty".to_string(),
            ));
        }
        if message.len() > 65536 {
            return Err(GroupChatError::InvalidInput(
                "Message too large (max 64KB)".to_string(),
            ));
        }

        // Verify sender is in the group
        if !self.has_participant(sender) {
            return Err(GroupChatError::InvalidInput(format!(
                "Sender {} is not in the group",
                sender
            )));
        }

        println!(
            "\n[{}] Broadcasting encrypted message: \"{}\"",
            sender, message
        );

        let shared_key = self.shared_secret.ok_or(GroupChatError::NotInitialized)?;

        let key = Key::<Aes256Gcm>::from_slice(&shared_key);
        let cipher = Aes256Gcm::new(key);

        // Generate cryptographically secure random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, message.as_bytes())
            .map_err(|e| GroupChatError::CryptoError(format!("Encryption failed: {}", e)))?;

        Ok((ciphertext, nonce_bytes.to_vec()))
    }

    /// Receive and decrypt a message
    pub fn receive(
        &self,
        receiver: &str,
        ciphertext: &[u8],
        nonce_bytes: &[u8],
    ) -> Result<String, GroupChatError> {
        // Input validation
        if receiver.is_empty() {
            return Err(GroupChatError::InvalidInput(
                "Receiver name cannot be empty".to_string(),
            ));
        }
        if ciphertext.is_empty() {
            return Err(GroupChatError::InvalidInput(
                "Ciphertext cannot be empty".to_string(),
            ));
        }
        if ciphertext.len() > 65536 + 16 {
            return Err(GroupChatError::InvalidInput(
                "Ciphertext too large".to_string(),
            ));
        }
        if nonce_bytes.len() != 12 {
            return Err(GroupChatError::InvalidInput(
                "Invalid nonce length".to_string(),
            ));
        }

        let shared_key = self.shared_secret.ok_or(GroupChatError::NotInitialized)?;

        let key = Key::<Aes256Gcm>::from_slice(&shared_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let decrypted = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| GroupChatError::CryptoError(format!("Decryption failed: {}", e)))?;

        let message = String::from_utf8(decrypted)
            .map_err(|e| GroupChatError::CryptoError(format!("Invalid UTF-8: {}", e)))?;

        println!("[{}] Received and decrypted: \"{}\"", receiver, message);

        Ok(message)
    }

    /// Get serializable tree nodes for persistence
    pub fn serialize_tree(&self) -> Vec<crate::ratchet_tree::SerializableNode> {
        self.tree.to_serializable()
    }

    /// Restore tree from serialized nodes
    pub fn deserialize_tree(&mut self, nodes: Vec<crate::ratchet_tree::SerializableNode>) {
        self.tree = RatchetTree::from_serializable(nodes);
        // Recalculate shared secret
        if let Ok(secret) = self.tree.derive_group_secret() {
            self.shared_secret = Some(secret);
        }
    }
}

impl Default for GroupChat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_group() {
        let mut chat = GroupChat::new();
        let result = chat.setup_group(vec!["Alice".to_string(), "Bob".to_string()]);
        assert!(result.is_ok());
        assert!(chat.shared_secret.is_some());
    }

    #[test]
    fn test_broadcast_receive() {
        let mut chat = GroupChat::new();
        chat.setup_group(vec!["Alice".to_string(), "Bob".to_string()])
            .unwrap();

        let (ciphertext, nonce) = chat.broadcast("Alice", "Hello, Bob!").unwrap();
        let decrypted = chat.receive("Bob", &ciphertext, &nonce).unwrap();
        assert_eq!(decrypted, "Hello, Bob!");
    }

    #[test]
    fn test_add_member() {
        let mut chat = GroupChat::new();
        chat.setup_group(vec!["Alice".to_string(), "Bob".to_string()])
            .unwrap();
        let secret1 = chat.shared_secret.unwrap();

        chat.add_member("Carol".to_string()).unwrap();
        let secret2 = chat.shared_secret.unwrap();

        // Secret should change
        assert_ne!(secret1, secret2);
    }

    #[test]
    fn test_remove_member() {
        let mut chat = GroupChat::new();
        chat.setup_group(vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
        ])
        .unwrap();
        let secret1 = chat.shared_secret.unwrap();

        chat.remove_member("Bob").unwrap();
        let secret2 = chat.shared_secret.unwrap();

        // Secret should change (forward secrecy)
        assert_ne!(secret1, secret2);
    }
}
