// Unit tests for encryption/decryption and memory save/load
use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    fn test_key() -> Key {
        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);
        *Key::from_slice(&key_bytes)
    }

    #[test]
    fn test_encrypt_decrypt_integrity() {
        let key = test_key();
        let mut memory = Memory::default();
        memory.items.insert("test1".to_string(), Note {
            value: "value1".to_string(),
            tags: vec!["tag1".to_string()],
            timestamp: chrono::Utc::now(),
        });
        let encrypted = encrypt_memory(&memory, &key).expect("encryption failed");
        let decrypted = decrypt_memory(&encrypted, &key).expect("decryption failed");
        assert_eq!(memory.items["test1"].value, decrypted.items["test1"].value);
        assert_eq!(memory.items["test1"].tags, decrypted.items["test1"].tags);
    }

    #[test]
    fn test_memory_save_load_various_types() {
        let key = test_key();
        let mut memory = Memory::default();
        let long_string = "a".repeat(1000);
        let cases = vec![
            ("int", "42"),
            ("float", "3.14"),
            ("unicode", "你好世界"),
            ("emoji", "🚀✨"),
            ("long", long_string.as_str()),
        ];
        for (k, v) in &cases {
            save_memory(&mut memory, &key, k, v, &[]).expect("save failed");
        }
        let encrypted = encrypt_memory(&memory, &key).expect("encryption failed");
        let loaded = decrypt_memory(&encrypted, &key).expect("decryption failed");
        for (k, v) in &cases {
            assert_eq!(loaded.items[*k].value, *v);
        }
    }
}
