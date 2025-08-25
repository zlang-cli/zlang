// Edge case tests for CLI
use super::*;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use rand::RngCore;
    use tempfile::tempdir;

    fn setup_master_key(path: &Path) -> Key {
        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);
        fs::write(path.join("master.key"), &key_bytes).unwrap();
        *Key::from_slice(&key_bytes)
    }

    #[test]
    fn test_save_empty_key_or_value() {
        let dir = tempdir().unwrap();
        let key = setup_master_key(dir.path());
        let mut memory = Memory::default();
        // Empty key
        let result = save_memory(&mut memory, &key, "", "value", &[]);
        assert!(result.is_ok(), "Should allow empty key");
        // Empty value
        let result = save_memory(&mut memory, &key, "key", "", &[]);
        assert!(result.is_ok(), "Should allow empty value");
        // Both empty
        let result = save_memory(&mut memory, &key, "", "", &[]);
        assert!(result.is_ok(), "Should allow both empty");
    }

    #[test]
    fn test_load_with_wrong_master_key() {
        let dir = tempdir().unwrap();
        let key1 = setup_master_key(dir.path());
        let mut memory = Memory::default();
        save_memory(&mut memory, &key1, "k1", "v1", &[]).unwrap();
        let encrypted = encrypt_memory(&memory, &key1).unwrap();
        fs::write(dir.path().join("memory.bin"), &encrypted).unwrap();
        // Generate a different key
        let mut wrong_key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut wrong_key_bytes);
        let wrong_key = *Key::from_slice(&wrong_key_bytes);
        let loaded = decrypt_memory(&encrypted, &wrong_key);
        assert!(loaded.is_err(), "Should fail to decrypt with wrong key");
    }

    #[test]
    fn test_recovery_file_in_fresh_env() {
        let dir = tempdir().unwrap();
        // Simulate onboarding and recovery file creation
        let recovery = RecoveryInfo {
            uuid: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        };
        let recovery_json = serde_json::to_string_pretty(&recovery).unwrap();
        fs::write(dir.path().join("recovery.json"), &recovery_json).unwrap();
        // Simulate fresh environment: no master.key or memory.bin
        let master_key_path = dir.path().join("master.key");
        let memory_path = dir.path().join("memory.bin");
        assert!(!master_key_path.exists());
        assert!(!memory_path.exists());
        // Load recovery file
        let loaded: RecoveryInfo = serde_json::from_str(&recovery_json).unwrap();
        assert_eq!(loaded.uuid, recovery.uuid);
    }
}
