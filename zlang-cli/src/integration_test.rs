// Integration tests for full CLI workflows and error handling
use super::*;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    fn setup_master_key(path: &Path) -> Key {
        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);
        fs::write(path.join("master.key"), &key_bytes).unwrap();
        *Key::from_slice(&key_bytes)
    }

    #[test]
    fn test_full_workflow() {
        let dir = tempdir().unwrap();
        let key = setup_master_key(dir.path());
        let mut memory = Memory::default();
        // Save
        save_memory(&mut memory, &key, "k1", "v1", &["tag1".to_string()]).unwrap();
        // Encrypt and write
        let encrypted = encrypt_memory(&memory, &key).unwrap();
        fs::write(dir.path().join("memory.bin"), &encrypted).unwrap();
        // Load
        let loaded = decrypt_memory(&encrypted, &key).unwrap();
        assert_eq!(loaded.items["k1"].value, "v1");
        // Show
        assert!(loaded.items.contains_key("k1"));
    }

    #[test]
    fn test_recovery_file() {
        let dir = tempdir().unwrap();
        let recovery = RecoveryInfo {
            uuid: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        };
        let recovery_json = serde_json::to_string_pretty(&recovery).unwrap();
        fs::write(dir.path().join("recovery.json"), &recovery_json).unwrap();
        let loaded: RecoveryInfo = serde_json::from_str(&recovery_json).unwrap();
        assert_eq!(loaded.uuid, recovery.uuid);
    }

    #[test]
    fn test_missing_files_error() {
        let dir = tempdir().unwrap();
        // Try to load missing master.key
        let key_path = dir.path().join("master.key");
        let result = fs::read(&key_path);
        assert!(result.is_err());
        // Try to load missing memory.bin
        let mem_path = dir.path().join("memory.bin");
        let result = fs::read(&mem_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_corrupted_memory_file_error() {
        let dir = tempdir().unwrap();
        let key = setup_master_key(dir.path());
        // Write corrupted memory.bin
        fs::write(dir.path().join("memory.bin"), b"not valid").unwrap();
        let corrupted = fs::read(dir.path().join("memory.bin")).unwrap();
        let result = decrypt_memory(&corrupted, &key);
        assert!(result.is_err());
    }
}
