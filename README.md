# HPair - Secure Group Messaging Library

[![Crate](https://img.shields.io/crates/v/hpair.svg)](https://crates.io/crates/hpair)
[![Documentation](https://docs.rs/hpair/badge.svg)](https://docs.rs/hpair)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

**HPair** is a production-ready Rust library implementing **MLS-style Tree-based Diffie-Hellman** for encrypted group messaging and key establishment. Built with modern cryptographic principles and designed for real-world deployment.

## 🔐 Security Architecture

### Core Security Properties
- **Tree-based DH**: MLS-style ratchet tree for group key agreement
- **X25519**: Elliptic curve Diffie-Hellman for key exchange
- **AES-GCM-256**: Authenticated encryption with cryptographically secure nonces
- **Forward Secrecy**: Keys automatically refresh when members are removed
- **O(log n) Operations**: Efficient member addition and removal
- **Timing Attack Resistance**: Constant-time cryptographic operations

### Cryptographic Primitives
- **Ratchet Tree**: Binary tree structure for group key derivation
- **X25519 ECDH**: Elliptic curve Diffie-Hellman at each tree node
- **SHA-256**: Key derivation from DH results
- **AES-GCM-256**: Message encryption with authenticated encryption

## 📊 How It Works

### Tree-based Key Agreement

```
          [Root: Group Secret]
                /          \
          [DH(L,R)]      [DH(L,R)]
           /    \          /    \
        Alice  Bob      Carol  Dave
         sk    sk        sk     sk
```

1. **Each participant** generates their own X25519 keypair (private + public)
2. **Internal nodes** are derived from DH(child_left_sk, child_right_pk)
3. **Root node** becomes the group secret
4. **All members** can independently derive the same group key

### Key Properties

| Property | Description |
|----------|-------------|
| **Real DH** | Each node requires a private key - not derivable from public keys alone |
| **Forward Secrecy** | When a member leaves, all keys are refreshed |
| **Efficient Updates** | O(log n) operations for add/remove |
| **Persistence** | Tree structure can be serialized for storage |

## 🏗️ API Architecture

### Public API Surface

```rust
┌─────────────────────────────────────────┐
│           HPair Public API              │
├─────────────────────────────────────────┤
│ • create_group(participants)            │
│ • send_encrypted_message(group, sender) │
│ • destroy_group(group_id)               │
│ • list_groups()                         │
│ • get_group_info(group_id)              │
└─────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────┐
│        Cryptographic Engine             │
├─────────────────────────────────────────┤
│ • Ratchet Tree (MLS-style)              │
│ • X25519 Diffie-Hellman                 │
│ • AES-GCM-256 Encryption                │
│ • Encrypted Persistent Storage          │
│ • Constant-Time Operations              │
└─────────────────────────────────────────┘
```

### Clean, Minimal API

```rust
use hpair::{create_group, send_encrypted_message};

// Create a secure group with tree-based key establishment
let participants = vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()];
let group_id = create_group(participants)?;

// Send encrypted messages with forward secrecy
send_encrypted_message(group_id, "alice", "Hello, secure group! 🔒")?;
```

## 🚀 Installation & Usage

### Add to Cargo.toml
```toml
[dependencies]
hpair = "0.1.4"
```

### Basic Usage Example

```rust
use hpair::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a secure group
    let participants = vec![
        "alice".to_string(),
        "bob".to_string(),
        "charlie".to_string()
    ];

    let group_id = create_group(participants)?;
    println!("Created secure group with ID: {}", group_id);

    // 2. Send encrypted messages
    send_encrypted_message(group_id, "alice", "Welcome to our secure group chat! 🔐")?;
    send_encrypted_message(group_id, "bob", "Thanks Alice! This is truly secure.")?;

    // 3. Check group information
    let (participants, created_at) = get_group_info(group_id)?;
    println!("Group has {} participants", participants.len());

    // 4. Clean up when done
    destroy_group(group_id)?;

    Ok(())
}
```

## 🔧 Advanced Configuration

### Environment Variables
```bash
# Custom storage directory (default: ~/.hpair)
export HPAIR_STORAGE_DIR="/custom/path/to/storage"

# Run with custom configuration
cargo run --release
```

### Storage Architecture
- **Encrypted File Storage**: AES-GCM-256 encrypted group data
- **Tree Persistence**: Ratchet tree structure serialized to disk
- **Automatic Cleanup**: Expired groups removed automatically
- **Resource Limits**: Configurable group count limits
- **Atomic Operations**: Crash-safe file operations

## 🌐 REST API Server (Optional)

HPair includes a complete REST API server for web applications:

```bash
# Start the API server
cargo run

# Server will be available at http://localhost:3000
```

### API Endpoints

```http
# Create a new group
POST /groups
Content-Type: application/json
{
  "participants": ["alice", "bob", "charlie"]
}

# Send encrypted message
POST /groups/{group_id}/messages
Content-Type: application/json
{
  "sender": "alice",
  "message": "Hello, secure world!"
}

# Get group information
GET /groups/{group_id}

# List all groups
GET /groups

# Destroy group
DELETE /groups/{group_id}
```

## 🧪 Testing & Validation

### Run the Test Suite
```bash
# Run all tests
cargo test

# Run ratchet tree tests
cargo test ratchet_tree

# Run with verbose output
cargo test -- --nocapture
```

### Security Validation
- ✅ **Memory Safety**: Comprehensive bounds checking, no unsafe code
- ✅ **Real DH**: Each tree node derived using private keys
- ✅ **Forward Secrecy**: Keys refresh on member removal
- ✅ **Input Validation**: All user inputs validated and sanitized
- ✅ **Error Handling**: No silent failures, comprehensive error reporting

## 🔬 Technical Details

### Ratchet Tree Construction

The ratchet tree uses heap-style array indexing:
- **Index 0**: Root node
- **Index 2i+1**: Left child of node i
- **Index 2i+2**: Right child of node i

### Key Derivation at Internal Nodes

```rust
// DH between children
let shared_secret = left_child_sk.diffie_hellman(right_child_pk);

// Derive new keypair for internal node
let hash = SHA256("ratchet-tree-derive-v1" || shared_secret);
let internal_sk = StaticSecret::from(hash);
let internal_pk = PublicKey::from(&internal_sk);
```

### Group Secret Derivation

```rust
// From root node
let group_secret = SHA256("group-secret-v1" || root_sk || root_pk);
```

## 🤝 Contributing

We welcome contributions! Please:

1. **Security First**: All changes undergo security review
2. **Comprehensive Tests**: Add tests for new functionality
3. **Documentation**: Update docs for API changes
4. **Performance**: Maintain or improve performance characteristics

### Development Setup
```bash
# Clone the repository
git clone https://github.com/nzengi/HPairing.git
cd HPairing

# Run tests
cargo test

# Build documentation
cargo doc
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Security Notice

**This software is designed with security-first principles but has not undergone formal cryptographic audit.** While implementing well-established cryptographic primitives (X25519, AES-GCM), it should be reviewed before production use.

### Recommended Usage
- ✅ **Educational purposes**
- ✅ **Research and development**
- ✅ **Prototyping secure messaging**
- ⚠️ **Production systems** (requires additional audit)

## 🔗 Links

- **Homepage**: https://github.com/nzengi/HPairing
- **Documentation**: https://docs.rs/hpair

---

**Built with ❤️ for secure group communication**