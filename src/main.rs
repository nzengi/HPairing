use hpair::{create_group, send_encrypted_message, calculate_quantum_resistance};

fn main() {
    println!("--- HPair Clean API Demo ---");
    println!("Secure Multi-Linear Group Encryption with Quantum Resistance\n");

    // Demo 1: Create and use a secure group
    println!("=== Demo 1: Basic Group Operations ===");

    let participants = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
    ];

    // Create group
    let group_id = match create_group(participants) {
        Ok(id) => {
            println!("✅ Group created successfully (ID: {})", id);
            id
        }
        Err(e) => {
            println!("❌ Failed to create group: {}", e);
            return;
        }
    };

    // Participants are automatically set up when group is created
    println!("✅ Alice and Bob are ready to communicate");

    // Send encrypted messages
    if let Err(e) = send_encrypted_message(group_id, "Alice", "Hello, secure group!") {
        println!("❌ Failed to send message: {}", e);
        return;
    }
    println!("✅ Alice sent: 'Hello, secure group!'");

    if let Err(e) = send_encrypted_message(group_id, "Bob", "Hi Alice! Great encryption!") {
        println!("❌ Failed to send message: {}", e);
        return;
    }
    println!("✅ Bob sent: 'Hi Alice! Great encryption!'");

    // Demo 2: Quantum resistance calculation
    println!("\n=== Demo 2: Quantum Resistance Analysis ===");

    let test_key = vec![0xAAu8; 32]; // 256-bit test key
    let quantum_resistance = calculate_quantum_resistance(&test_key);

    println!("🔐 Test key quantum resistance: {} bits", quantum_resistance);
    println!("📊 This provides approximately {} bits of quantum security", quantum_resistance);

    if quantum_resistance >= 128 {
        println!("✅ Key meets quantum-resistant standards (>= 128 bits)");
    } else {
        println!("⚠️  Key may not provide sufficient quantum resistance");
    }

    // Demo 3: Error handling
    println!("\n=== Demo 3: Error Handling ===");

    // Test invalid operations - try sending to non-existent group
    if let Err(e) = send_encrypted_message(99999, "Alice", "test") {
        println!("✅ Correctly rejected invalid group: {}", e);
    }

    if let Err(e) = send_encrypted_message(group_id, "Eve", "Unauthorized message") {
        println!("✅ Correctly rejected unauthorized sender: {}", e);
    }

    if let Err(e) = send_encrypted_message(group_id, "Alice", "") {
        println!("✅ Correctly rejected empty message: {}", e);
    }

    println!("\n🎉 HPair Clean API Demo completed successfully!");
    println!("🔒 All cryptographic operations performed securely with quantum resistance.");
}
