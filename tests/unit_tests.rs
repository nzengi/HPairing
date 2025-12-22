#[cfg(test)]
mod tests {
    use hpair::{create_group, send_encrypted_message, calculate_quantum_resistance, destroy_group, HPairError};

    #[test]
    fn test_create_group_success() {
        let participants = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
        let result = create_group(participants);

        assert!(result.is_ok());
        let group_id = result.unwrap();
        assert!(group_id > 0);
    }

    #[test]
    fn test_create_group_empty_participants() {
        let participants = vec![];
        let result = create_group(participants);

        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::GroupCreationFailed => {},
            _ => panic!("Expected GroupCreationFailed error"),
        }
    }

    #[test]
    fn test_create_group_duplicate_participants() {
        let participants = vec!["Alice".to_string(), "Alice".to_string()];
        let result = create_group(participants);

        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::InvalidParticipant => {},
            _ => panic!("Expected InvalidParticipant error"),
        }
    }


    #[test]
    fn test_send_encrypted_message_success() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        // Participants are automatically set up when group is created
        let result = send_encrypted_message(group_id, "Alice", "Hello, Bob!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_encrypted_message_group_not_found() {
        let result = send_encrypted_message(99999, "Alice", "Hello!");
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::GroupNotFound => {},
            _ => panic!("Expected GroupNotFound error"),
        }
    }

    #[test]
    fn test_send_encrypted_message_participant_not_found() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        let result = send_encrypted_message(group_id, "Charlie", "Hello!");
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::ParticipantNotFound => {},
            _ => panic!("Expected ParticipantNotFound error"),
        }
    }

    #[test]
    fn test_send_encrypted_message_empty_sender() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        let result = send_encrypted_message(group_id, "", "Hello!");
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::InvalidParticipant => {},
            _ => panic!("Expected InvalidParticipant error"),
        }
    }

    #[test]
    fn test_send_encrypted_message_empty_message() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        let result = send_encrypted_message(group_id, "Alice", "");
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::MessageEmpty => {},
            _ => panic!("Expected MessageEmpty error"),
        }
    }

    #[test]
    fn test_calculate_quantum_resistance() {
        // Test with a known good key (32 bytes of 0xFF)
        let key = vec![0xFFu8; 32];
        let resistance = calculate_quantum_resistance(&key);
        // For 0xFF repeated bytes, entropy is very low, so resistance will be low
        assert!(resistance >= 0);
        assert!(resistance <= 256); // Should not exceed key entropy
    }

    #[test]
    fn test_calculate_quantum_resistance_empty() {
        let key = vec![];
        let resistance = calculate_quantum_resistance(&key);
        assert_eq!(resistance, 0);
    }

    #[test]
    fn test_calculate_quantum_resistance_low_entropy() {
        // Test with very low entropy (all zeros)
        let key = vec![0u8; 32];
        let resistance = calculate_quantum_resistance(&key);
        assert!(resistance < 50); // Should be very low due to no entropy
    }

    #[test]
    fn test_full_workflow() {
        // Test complete workflow: create group, join, send messages
        let participants = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
        let group_id = create_group(participants).unwrap();

        // Participants are automatically set up when group is created

        // Send messages
        send_encrypted_message(group_id, "Alice", "Hello, group!").unwrap();
        send_encrypted_message(group_id, "Bob", "Hi Alice!").unwrap();

        // Test quantum resistance calculation
        let dummy_key = vec![0xAAu8; 32];
        let resistance = calculate_quantum_resistance(&dummy_key);
        assert!(resistance >= 64); // Should be at least minimum quantum security
    }

    #[test]
    fn test_destroy_group_success() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        let result = destroy_group(group_id);
        assert!(result.is_ok());

        // Verify group is gone
        let send_result = send_encrypted_message(group_id, "Alice", "test");
        assert!(send_result.is_err());
        match send_result.unwrap_err() {
            HPairError::GroupNotFound => {},
            _ => panic!("Expected GroupNotFound error"),
        }
    }

    #[test]
    fn test_destroy_group_not_found() {
        let result = destroy_group(99999);
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::GroupNotFound => {},
            _ => panic!("Expected GroupNotFound error"),
        }
    }

    #[test]
    fn test_input_validation() {
        // Test invalid participant names
        let participants = vec!["".to_string()]; // Empty name
        let result = create_group(participants);
        assert!(result.is_err());

        let participants = vec!["Alice@domain.com".to_string()]; // Invalid characters
        let result = create_group(participants);
        assert!(result.is_err());

        let long_name = "A".repeat(100); // Too long
        let participants = vec![long_name];
        let result = create_group(participants);
        assert!(result.is_err());

        // Test invalid sender name
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        let result = send_encrypted_message(group_id, "Alice@hack", "test");
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::InvalidParticipant => {},
            _ => panic!("Expected InvalidParticipant error"),
        }
    }

    #[test]
    fn test_message_size_limits() {
        let participants = vec!["Alice".to_string(), "Bob".to_string()];
        let group_id = create_group(participants).unwrap();

        // Test oversized message
        let large_message = "A".repeat(70000);
        let result = send_encrypted_message(group_id, "Alice", &large_message);
        assert!(result.is_err());
        match result.unwrap_err() {
            HPairError::MessageTooLarge => {},
            _ => panic!("Expected MessageTooLarge error"),
        }
    }
}
