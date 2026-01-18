# Security Analysis - HPair Cryptographic Library

## 🔐 Security Model

HPair implements an **MLS-style Ratchet Tree** protocol for group key establishment. The security relies on the hardness of the Elliptic Curve Diffie-Hellman (ECDH) problem using Curve25519.

### Security Assumptions

1. **Computational Diffie-Hellman (CDH)** hardness on Curve25519
2. **Random Oracle Model** for SHA-256 key derivation
3. **IND-CCA2 Security** of AES-GCM-256

## 🎯 Threat Model

### Attack Vectors Considered

#### 1. Passive Eavesdropping

- **Mitigation**: AES-GCM-256 encryption with unique nonces
- **Assurance**: Each message uses cryptographically secure random nonces

#### 2. Active Man-in-the-Middle

- **Mitigation**: Tree-based key agreement requires possession of private keys
- **Assurance**: Only legitimate group members can derive the group secret

#### 3. Key Compromise (Forward Secrecy)

- **Mitigation**: Full tree refresh when members are removed
- **Assurance**: Compromised keys don't reveal past session keys

#### 4. Side-Channel Attacks

- **Mitigation**: Constant-time X25519 operations via x25519-dalek
- **Assurance**: Library uses constant-time implementations

#### 5. Member Removal Security

- **Mitigation**: Complete key refresh using fresh randomness
- **Assurance**: Removed members cannot decrypt future messages

## 🌳 Tree-based Key Agreement Security

### How It Works

```
        [Root: Group Secret]
              /          \
        [DH(L,R)]      [DH(L,R)]
         /    \          /    \
      Alice  Bob      Carol  Dave
       sk    sk        sk     sk
```

### Security Properties

| Property | Guarantee |
|----------|-----------|
| **Confidentiality** | Only members with private keys can derive group secret |
| **Forward Secrecy** | Tree refresh on member removal prevents past key derivation |
| **Key Independence** | Different groups have independent keys |
| **Efficient Updates** | O(log n) operations maintain security during changes |

### Why Public Keys Alone Are Insufficient

In the ratchet tree, internal nodes are computed as:

```
internal_node = DH(left_child_sk, right_child_pk)
```

This requires possession of at least one child's **private key**. An attacker with only public keys cannot compute the DH result or derive the group secret.

## 📊 Security Parameters

| Parameter | Value | Security Level | Rationale |
|-----------|-------|----------------|-----------|
| Key Exchange | X25519 | 128-bit | ECDH on Curve25519 |
| Encryption | AES-GCM-256 | 256-bit | NIST recommended |
| Key Derivation | SHA-256 | 256-bit | Collision resistant |
| Nonce Size | 96-bit | 128-bit | GCM security bound |

## 🔍 Security Audit Markers

### Critical Security Points

```rust
// SECURITY: Each participant generates their own keypair
let secret = StaticSecret::random_from_rng(OsRng);
let public = PublicKey::from(&secret);
```

```rust
// SECURITY: Internal nodes require private key for DH
let shared_secret = left_sk.diffie_hellman(right_pk);
```

```rust
// SECURITY: Tree refresh on member removal (forward secrecy)
fn refresh_tree(&mut self) {
    // Re-randomize all leaves with new keys
    for leaf in leaves {
        leaf.private_key = StaticSecret::random_from_rng(OsRng);
    }
    self.rebuild_internal_nodes();
}
```

```rust
// SECURITY: Generate cryptographically secure random nonce
let mut nonce_bytes = [0u8; 12];
OsRng.fill_bytes(&mut nonce_bytes);
```

## 🧪 Security Testing

### Automated Tests

```bash
# Run all security-focused tests
cargo test

# Run ratchet tree tests
cargo test ratchet_tree

# Verify forward secrecy
cargo test test_remove_member
```

### Security Test Coverage

- ✅ **Key Generation**: X25519 keypair generation
- ✅ **DH Computation**: Correct shared secret derivation
- ✅ **Tree Construction**: Proper internal node computation
- ✅ **Forward Secrecy**: Key change on member removal
- ✅ **Serialization**: Round-trip preserves security properties
- ✅ **Input Validation**: Bounds checking and sanitization
- ✅ **Error Handling**: No information leakage through errors

## 🚨 Known Limitations

### Current Limitations

1. **Server Key Knowledge**: In the current implementation, the server generates and stores all private keys. For true E2E encryption, participants should generate keys client-side.

2. **No Authentication**: REST API lacks authentication - anyone can access endpoints.

3. **No Message Signing**: Messages are encrypted but not signed - no non-repudiation.

4. **Synchronous Tree Updates**: All members must be online for tree updates (not fully async).

### Future Security Enhancements

- [ ] Client-side key generation
- [ ] API authentication (JWT/API keys)
- [ ] Message signing for non-repudiation
- [ ] Asynchronous tree update protocol
- [ ] Hardware security module (HSM) support

## 📞 Security Contact

For security vulnerabilities or concerns:

1. **DO NOT** create public GitHub issues for security problems
2. Email security concerns to: howyaniii@gmail.com
3. Allow 48 hours for initial response

## 🔄 Security Updates

### Version Security Status

| Version | Status | Known Issues | Recommended Action |
|---------|--------|--------------|-------------------|
| 0.1.4 | 🟡 Beta | Server stores keys | Research use only |
| 1.0.0 | 🟢 Planned | - | Production ready |

### Security Update Process

1. Critical vulnerabilities: Hotfix within 24 hours
2. High severity: Patch within 1 week
3. Medium severity: Patch within 1 month
4. Low severity: Address in next minor version

## 📚 Security References

- [RFC 7748: Elliptic Curves for Security (X25519)](https://tools.ietf.org/html/rfc7748)
- [RFC 9420: Messaging Layer Security (MLS)](https://datatracker.ietf.org/doc/html/rfc9420)
- [NIST SP 800-38D: GCM Mode](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistsp800-38d.pdf)

## ✅ Security Checklist

- [x] X25519 for key exchange (well-studied, constant-time)
- [x] AES-GCM-256 for encryption (NIST approved)
- [x] SHA-256 for key derivation
- [x] Cryptographically secure random number generation
- [x] Forward secrecy via tree refresh
- [x] Input validation and sanitization
- [x] Memory safety (Rust guarantees)
- [x] No hardcoded secrets or keys
- [ ] Client-side key generation (planned)
- [ ] API authentication (planned)
- [ ] Message signing (planned)
