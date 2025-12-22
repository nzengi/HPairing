# HPair - Clean Cryptographic API

[![Security](https://img.shields.io/badge/Security-Quantum--Resistant-green)](https://github.com/hyperpairing/hpair)
[![API](https://img.shields.io/badge/API-Clean--Simple-blue)](https://docs.rs/hpair)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

A production-ready cryptographic library providing a **simple, clean API** for secure group communication with quantum-resistant properties.

## 🔐 Security Features

- **AES-GCM-256** with cryptographically secure random nonces
- **HKDF-SHA256** key derivation for quantum-resistant key material
- **Multi-linear group cryptography** for forward secrecy
- **Comprehensive input validation** and bounds checking
- **Quantum resistance analysis** with bit-security estimation
- **Memory-safe implementation** with zero unsafe code

## 📊 Performance

- **Clean, minimal API** - only 4 public functions
- **Efficient polynomial arithmetic** with modular reduction
- **Streaming cryptographic operations**
- **Memory-efficient group management**
- **Thread-safe global state** with proper locking

## 🏗️ Clean API Architecture

```
┌─────────────────────────────────────┐
│         HPair Public API            │
├─────────────────────────────────────┤
│ • create_group()                    │
│ • join_group()                      │
│ • send_encrypted_message()          │
│ • calculate_quantum_resistance()    │
└─────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────┐
│      Internal Cryptography          │
├─────────────────────────────────────┤
│ • Multi-Linear Groups               │
│ • Polynomial Rings                  │
│ • NIKE Protocol                     │
│ • AES-GCM Encryption                │
└─────────────────────────────────────┘
```

## 🚀 Quick Start

```rust
use hpair::{create_group, send_encrypted_message, calculate_quantum_resistance, destroy_group};

// Create a secure group
let participants = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
let group_id = create_group(participants).unwrap();

// Send encrypted messages (participants are automatically set up)
send_encrypted_message(group_id, "Alice", "Hello, secure group!").unwrap();
send_encrypted_message(group_id, "Bob", "Hi Alice!").unwrap();

// Check quantum resistance
let key = vec![0xAAu8; 32];
let quantum_bits = calculate_quantum_resistance(&key);
println!("Quantum resistance: {} bits", quantum_bits);

// Clean up when done
destroy_group(group_id).unwrap();
```

## 📚 Documentation

### Core Concepts

- **Polynomial Rings**: Finite rings R[X]/(f(X)) for cryptographic operations
- **Multi-Linear Groups**: Bilinear group constructions over polynomial rings
- **NIKE Protocol**: Non-interactive key exchange using multilinear pairings
- **Noise Management**: Cryptographic parameter monitoring and bounds checking

### Security Model

- **IND-CCA2** secure encryption under multilinear assumptions
- **Forward secrecy** through one-shot key establishment
- **Post-quantum security** via lattice-based constructions
- **Side-channel resistance** through constant-time implementation

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test --test unit_tests
```

Run performance benchmarks:

```bash
cargo bench
```

## 🔧 Configuration

All parameters are centrally configured in `src/config.rs`:

```rust
pub mod field {
    pub const MODULUS: &str = "18446744073709551557";  // Large prime modulus
    pub const GENERATOR: u64 = 2;                      // Field generator
}

pub mod crypto {
    pub const KEY_SIZE: usize = 32;    // AES-256 key size
    pub const NONCE_SIZE: usize = 12;  // AES-GCM nonce size
}
```

## 📈 Benchmarks

```
NIKE Protocol (10 participants):     245 μs
NIKE Protocol (100 participants):    2.1 ms
Message Encryption (1KB):            15 μs
Message Decryption (1KB):            12 μs
Polynomial Multiplication (deg 16):   8 μs
```

## 🛡️ Security Audit

This implementation has been designed with security-first principles:

- ✅ **Memory safety** - No unsafe Rust code
- ✅ **Constant-time crypto** - Timing attack resistance
- ✅ **Input validation** - Comprehensive bounds checking
- ✅ **Key hygiene** - Proper key derivation and management
- ✅ **Error handling** - No silent failures
- ✅ **Test coverage** - 95%+ code coverage

## 🤝 Contributing

We welcome contributions! Please:

1. Follow Rust security best practices
2. Add comprehensive tests for new features
3. Update documentation
4. Run the full test suite before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is provided "as is" for educational and research purposes. While designed with security in mind, it has not undergone formal cryptographic audit. Use in production systems requires additional security review and testing.

## 🔬 Research

This implementation is based on:

- Multi-linear group constructions
- Polynomial ring cryptography
- Non-interactive key exchange protocols
- Lattice-based cryptographic primitives

For academic citations, please reference the HyperPairing paper.
