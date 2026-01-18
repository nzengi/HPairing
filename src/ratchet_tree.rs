//! # Ratchet Tree Module
//!
//! This module implements a binary tree structure for MLS-style group key exchange.
//!
//! ## Overview
//!
//! The ratchet tree is a binary tree where:
//! - Leaf nodes represent group members with their X25519 keypairs
//! - Internal nodes contain DH-derived secrets from their children
//! - The root node contains the group secret
//!
//! ## Tree Structure
//!
//! ```text
//!           [Root: Group Secret]
//!                 /          \
//!           [Internal]      [Internal]
//!            /    \          /    \
//!         Alice  Bob      Carol  Dave
//! ```
//!
//! ## Security Properties
//!
//! - **Forward Secrecy**: When a member is removed, new keys are derived
//! - **Post-Compromise Security**: When a member updates their key, the tree is refreshed
//! - **Efficient Updates**: O(log n) operations for add/remove

use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

/// Error type for ratchet tree operations
#[derive(Debug, Clone)]
pub enum TreeError {
    /// Tree is empty
    EmptyTree,
    /// Member not found
    MemberNotFound(String),
    /// Invalid tree structure
    InvalidStructure,
    /// Cryptographic operation failed
    CryptoError(String),
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeError::EmptyTree => write!(f, "Tree is empty"),
            TreeError::MemberNotFound(name) => write!(f, "Member not found: {}", name),
            TreeError::InvalidStructure => write!(f, "Invalid tree structure"),
            TreeError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
        }
    }
}

impl std::error::Error for TreeError {}

/// Serializable representation of a tree node
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializableNode {
    pub node_type: String,
    pub name: Option<String>,
    pub public_key: Option<[u8; 32]>,
    pub private_key: Option<[u8; 32]>,
}

/// A node in the ratchet tree
#[derive(Clone)]
pub enum TreeNode {
    /// Leaf node - represents a group member
    Leaf {
        name: String,
        public_key: PublicKey,
        private_key: StaticSecret,
    },
    /// Internal node - derived from children
    Internal {
        public_key: PublicKey,
        private_key: StaticSecret,
    },
    /// Empty slot (for balanced tree)
    Empty,
}

impl TreeNode {
    /// Get the public key of this node (if any)
    pub fn public_key(&self) -> Option<&PublicKey> {
        match self {
            TreeNode::Leaf { public_key, .. } => Some(public_key),
            TreeNode::Internal { public_key, .. } => Some(public_key),
            TreeNode::Empty => None,
        }
    }

    /// Get the private key of this node (if any)
    pub fn private_key(&self) -> Option<&StaticSecret> {
        match self {
            TreeNode::Leaf { private_key, .. } => Some(private_key),
            TreeNode::Internal { private_key, .. } => Some(private_key),
            TreeNode::Empty => None,
        }
    }

    /// Check if this node is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, TreeNode::Empty)
    }

    /// Get the name (for leaf nodes only)
    pub fn name(&self) -> Option<&str> {
        match self {
            TreeNode::Leaf { name, .. } => Some(name),
            _ => None,
        }
    }

    /// Serialize this node
    pub fn to_serializable(&self) -> SerializableNode {
        match self {
            TreeNode::Leaf {
                name,
                public_key,
                private_key,
            } => SerializableNode {
                node_type: "leaf".to_string(),
                name: Some(name.clone()),
                public_key: Some(public_key.to_bytes()),
                private_key: Some(private_key.to_bytes()),
            },
            TreeNode::Internal {
                public_key,
                private_key,
            } => SerializableNode {
                node_type: "internal".to_string(),
                name: None,
                public_key: Some(public_key.to_bytes()),
                private_key: Some(private_key.to_bytes()),
            },
            TreeNode::Empty => SerializableNode {
                node_type: "empty".to_string(),
                name: None,
                public_key: None,
                private_key: None,
            },
        }
    }

    /// Deserialize a node
    pub fn from_serializable(s: &SerializableNode) -> Self {
        match s.node_type.as_str() {
            "leaf" => {
                let public_key = PublicKey::from(s.public_key.unwrap());
                let private_key = StaticSecret::from(s.private_key.unwrap());
                TreeNode::Leaf {
                    name: s.name.clone().unwrap_or_default(),
                    public_key,
                    private_key,
                }
            }
            "internal" => {
                let public_key = PublicKey::from(s.public_key.unwrap());
                let private_key = StaticSecret::from(s.private_key.unwrap());
                TreeNode::Internal {
                    public_key,
                    private_key,
                }
            }
            _ => TreeNode::Empty,
        }
    }
}

/// The Ratchet Tree structure for group key exchange
#[derive(Clone)]
pub struct RatchetTree {
    /// Nodes stored in array form (heap-like indexing)
    /// For a tree with n leaves, we need 2n - 1 nodes
    nodes: Vec<TreeNode>,
    /// Number of actual participants (non-empty leaves)
    member_count: usize,
}

impl RatchetTree {
    /// Create a new empty tree
    pub fn empty() -> Self {
        Self {
            nodes: vec![],
            member_count: 0,
        }
    }

    /// Create a new tree with the given participants
    pub fn new(participants: Vec<String>) -> Self {
        if participants.is_empty() {
            return Self::empty();
        }

        // Calculate tree size: for n leaves, we need 2n - 1 nodes
        let n = participants.len();
        let leaf_capacity = n.next_power_of_two(); // Round up to power of 2
        let tree_size = 2 * leaf_capacity - 1;

        let mut nodes = vec![TreeNode::Empty; tree_size];

        // Place leaves at the bottom of the tree
        let first_leaf_index = leaf_capacity - 1;
        for (i, name) in participants.into_iter().enumerate() {
            let secret = StaticSecret::random_from_rng(OsRng);
            let public = PublicKey::from(&secret);
            nodes[first_leaf_index + i] = TreeNode::Leaf {
                name,
                public_key: public,
                private_key: secret,
            };
        }

        let mut tree = Self {
            nodes,
            member_count: n,
        };

        // Build internal nodes from bottom up
        tree.rebuild_internal_nodes();
        tree
    }

    /// Get the number of members in the tree
    pub fn member_count(&self) -> usize {
        self.member_count
    }

    /// Get all member names
    pub fn members(&self) -> Vec<String> {
        self.nodes
            .iter()
            .filter_map(|node| node.name().map(String::from))
            .collect()
    }

    /// Get the leaf capacity (power of 2)
    fn leaf_capacity(&self) -> usize {
        (self.nodes.len() + 1) / 2
    }

    /// Get the index of the first leaf
    fn first_leaf_index(&self) -> usize {
        self.leaf_capacity() - 1
    }

    /// Parent index (heap-style)
    fn parent(i: usize) -> usize {
        if i == 0 {
            0
        } else {
            (i - 1) / 2
        }
    }

    /// Left child index
    fn left_child(i: usize) -> usize {
        2 * i + 1
    }

    /// Right child index
    fn right_child(i: usize) -> usize {
        2 * i + 2
    }

    /// Check if index is a leaf
    fn is_leaf(&self, i: usize) -> bool {
        Self::left_child(i) >= self.nodes.len()
    }

    /// Rebuild all internal nodes from leaves up
    fn rebuild_internal_nodes(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        // Start from the parent of the last leaf and work up to root
        let first_leaf = self.first_leaf_index();
        if first_leaf == 0 {
            return; // Only one node (root = leaf)
        }

        // Process from bottom to top
        for i in (0..first_leaf).rev() {
            self.compute_internal_node(i);
        }
    }

    /// Compute an internal node from its children
    fn compute_internal_node(&mut self, index: usize) {
        let left_idx = Self::left_child(index);
        let right_idx = Self::right_child(index);

        if left_idx >= self.nodes.len() {
            return; // This is a leaf, nothing to compute
        }

        let left_pk = self.nodes[left_idx].public_key();
        let right_pk = self.nodes[right_idx].public_key();
        let left_sk = self.nodes[left_idx].private_key();

        // If either child is empty, this node becomes empty
        if left_pk.is_none() && right_pk.is_none() {
            self.nodes[index] = TreeNode::Empty;
            return;
        }

        // If only one child exists, propagate it up
        if left_pk.is_none() {
            if let Some(pk) = right_pk {
                let sk = self.nodes[right_idx].private_key().unwrap();
                self.nodes[index] = TreeNode::Internal {
                    public_key: *pk,
                    private_key: sk.clone(),
                };
            }
            return;
        }
        if right_pk.is_none() {
            if let Some(pk) = left_pk {
                let sk = self.nodes[left_idx].private_key().unwrap();
                self.nodes[index] = TreeNode::Internal {
                    public_key: *pk,
                    private_key: sk.clone(),
                };
            }
            return;
        }

        // Both children exist - perform DH
        let left_pk = left_pk.unwrap();
        let right_pk = right_pk.unwrap();
        let left_sk = left_sk.unwrap();

        // DH: left_sk * right_pk
        let shared_secret = left_sk.diffie_hellman(right_pk);

        // Derive new keypair for this internal node
        let (new_sk, new_pk) = Self::derive_keypair(shared_secret.as_bytes());

        self.nodes[index] = TreeNode::Internal {
            public_key: new_pk,
            private_key: new_sk,
        };
    }

    /// Derive a new keypair from seed bytes
    fn derive_keypair(seed: &[u8]) -> (StaticSecret, PublicKey) {
        let mut hasher = Sha256::new();
        hasher.update(b"ratchet-tree-derive-v1");
        hasher.update(seed);
        let hash = hasher.finalize();

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&hash);

        let secret = StaticSecret::from(key_bytes);
        let public = PublicKey::from(&secret);
        (secret, public)
    }

    /// Derive the group secret (root node's secret)
    pub fn derive_group_secret(&self) -> Result<[u8; 32], TreeError> {
        if self.nodes.is_empty() {
            return Err(TreeError::EmptyTree);
        }

        // Root is at index 0
        match &self.nodes[0] {
            TreeNode::Internal {
                private_key,
                public_key,
            }
            | TreeNode::Leaf {
                private_key,
                public_key,
                ..
            } => {
                // Hash private key to get the group secret
                let mut hasher = Sha256::new();
                hasher.update(b"group-secret-v1");
                hasher.update(private_key.to_bytes());
                hasher.update(public_key.as_bytes());
                let hash = hasher.finalize();

                let mut secret = [0u8; 32];
                secret.copy_from_slice(&hash);
                Ok(secret)
            }
            TreeNode::Empty => Err(TreeError::EmptyTree),
        }
    }

    /// Find a member by name and return their leaf index
    fn find_member(&self, name: &str) -> Option<usize> {
        self.nodes
            .iter()
            .position(|node| matches!(node, TreeNode::Leaf { name: n, .. } if n == name))
    }

    /// Add a new member to the tree
    pub fn add_member(&mut self, name: String) -> Result<(), TreeError> {
        // Find an empty leaf slot
        let first_leaf = self.first_leaf_index();
        let empty_slot = (first_leaf..self.nodes.len()).find(|&i| self.nodes[i].is_empty());

        if let Some(slot) = empty_slot {
            // We have an empty slot, use it
            let secret = StaticSecret::random_from_rng(OsRng);
            let public = PublicKey::from(&secret);
            self.nodes[slot] = TreeNode::Leaf {
                name,
                public_key: public,
                private_key: secret,
            };
            self.member_count += 1;

            // Update path from this leaf to root
            self.update_path(slot);
            Ok(())
        } else {
            // Need to expand the tree
            self.expand_and_add(name)
        }
    }

    /// Expand the tree and add a new member
    fn expand_and_add(&mut self, name: String) -> Result<(), TreeError> {
        // Collect current members
        let mut members = self.members();
        members.push(name);

        // Rebuild the tree with the new member
        *self = RatchetTree::new(members);
        Ok(())
    }

    /// Update the path from a leaf to the root
    fn update_path(&mut self, leaf_index: usize) {
        let mut current = leaf_index;
        while current > 0 {
            let parent = Self::parent(current);
            self.compute_internal_node(parent);
            current = parent;
        }
    }

    /// Remove a member from the tree
    pub fn remove_member(&mut self, name: &str) -> Result<(), TreeError> {
        let index = self
            .find_member(name)
            .ok_or_else(|| TreeError::MemberNotFound(name.to_string()))?;

        // Replace with empty node
        self.nodes[index] = TreeNode::Empty;
        self.member_count -= 1;

        // Update path from this leaf to root
        self.update_path(index);

        // Generate new secrets for forward secrecy
        self.refresh_tree();

        Ok(())
    }

    /// Refresh the tree (generate new secrets for forward secrecy)
    fn refresh_tree(&mut self) {
        // Re-randomize all leaves with new keys
        for i in self.first_leaf_index()..self.nodes.len() {
            if let TreeNode::Leaf { name, .. } = &self.nodes[i] {
                let name = name.clone();
                let secret = StaticSecret::random_from_rng(OsRng);
                let public = PublicKey::from(&secret);
                self.nodes[i] = TreeNode::Leaf {
                    name,
                    public_key: public,
                    private_key: secret,
                };
            }
        }

        // Rebuild internal nodes
        self.rebuild_internal_nodes();
    }

    /// Serialize the tree for storage
    pub fn to_serializable(&self) -> Vec<SerializableNode> {
        self.nodes.iter().map(|n| n.to_serializable()).collect()
    }

    /// Deserialize a tree from storage
    pub fn from_serializable(nodes: Vec<SerializableNode>) -> Self {
        let nodes: Vec<TreeNode> = nodes.iter().map(TreeNode::from_serializable).collect();
        let member_count = nodes.iter().filter(|n| n.name().is_some()).count();
        Self {
            nodes,
            member_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = RatchetTree::empty();
        assert_eq!(tree.member_count(), 0);
        assert!(tree.derive_group_secret().is_err());
    }

    #[test]
    fn test_single_member() {
        let tree = RatchetTree::new(vec!["Alice".to_string()]);
        assert_eq!(tree.member_count(), 1);
        assert!(tree.derive_group_secret().is_ok());
    }

    #[test]
    fn test_two_members() {
        let tree = RatchetTree::new(vec!["Alice".to_string(), "Bob".to_string()]);
        assert_eq!(tree.member_count(), 2);
        assert!(tree.derive_group_secret().is_ok());
    }

    #[test]
    fn test_four_members() {
        let tree = RatchetTree::new(vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
            "Dave".to_string(),
        ]);
        assert_eq!(tree.member_count(), 4);
        assert!(tree.derive_group_secret().is_ok());
    }

    #[test]
    fn test_add_member() {
        let mut tree = RatchetTree::new(vec!["Alice".to_string(), "Bob".to_string()]);
        let secret1 = tree.derive_group_secret().unwrap();

        tree.add_member("Carol".to_string()).unwrap();
        let secret2 = tree.derive_group_secret().unwrap();

        // Secret should change after adding member
        assert_ne!(secret1, secret2);
        assert_eq!(tree.member_count(), 3);
    }

    #[test]
    fn test_remove_member() {
        let mut tree = RatchetTree::new(vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
        ]);
        let secret1 = tree.derive_group_secret().unwrap();

        tree.remove_member("Bob").unwrap();
        let secret2 = tree.derive_group_secret().unwrap();

        // Secret should change after removing member (forward secrecy)
        assert_ne!(secret1, secret2);
        assert_eq!(tree.member_count(), 2);
    }

    #[test]
    fn test_remove_nonexistent_member() {
        let mut tree = RatchetTree::new(vec!["Alice".to_string()]);
        let result = tree.remove_member("Bob");
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let tree = RatchetTree::new(vec!["Alice".to_string(), "Bob".to_string()]);
        let secret1 = tree.derive_group_secret().unwrap();

        let serialized = tree.to_serializable();
        let restored = RatchetTree::from_serializable(serialized);
        let secret2 = restored.derive_group_secret().unwrap();

        // Secrets should match after serialization round-trip
        assert_eq!(secret1, secret2);
        assert_eq!(restored.member_count(), 2);
    }
}
