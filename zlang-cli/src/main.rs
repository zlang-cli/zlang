#[cfg(test)]
mod edge_case_test;
#[cfg(test)]
mod integration_test;
#[cfg(test)]
mod main_test;
fn network_allowed() -> Result<bool> {
	if !std::path::Path::new(NETWORK_CONFIG_FILE).exists() {
		return Ok(false);
	}
	let flag = std::fs::read_to_string(NETWORK_CONFIG_FILE)?;
	Ok(flag.trim() == "y")
}

fn network_test() -> Result<()> {
	println!("--- Network Test ---");
	let url = "https://api.github.com/zen";
	let client = match ureq::AgentBuilder::new().timeout(Duration::from_secs(5)).build().get(url)
		.set("User-Agent", "zlang-cli")
		.call() {
		Ok(resp) => resp,
		Err(e) => {
			println!("Network error: {}", e);
			return Ok(());
		}
	};
	let text = match client.into_string() {
		Ok(t) => t,
		Err(e) => {
			println!("Failed to read response: {}", e);
			return Ok(());
		}
	};
	println!("Fetched: {}", text);
	Ok(())
}

fn recover_from_file() -> Result<()> {
	println!("--- Recovery ---");
	if !std::path::Path::new(RECOVERY_FILE).exists() {
		println!("Recovery file not found: {}", RECOVERY_FILE);
		return Ok(());
	}
	let recovery_json = std::fs::read_to_string(RECOVERY_FILE)?;
	let recovery: RecoveryInfo = serde_json::from_str(&recovery_json)?;
	println!("Recovery file loaded. UUID: {} Timestamp: {}", recovery.uuid, recovery.timestamp);
	// Generate new master key
	let mut rng = ChaCha20Rng::from_entropy();
	let mut key_bytes = [0u8; 32];
	rng.fill_bytes(&mut key_bytes);
	std::fs::write(MASTER_KEY_FILE, &key_bytes)?;
	// Prompt for backup memory file
	println!("If you have a backup of your previous memory file, enter its path to restore it now.");
	println!("Otherwise, just press Enter to continue with an empty memory file.");
	print!("Backup memory file path: ");
	std::io::stdout().flush()?;
	let mut backup_path = String::new();
	std::io::stdin().read_line(&mut backup_path)?;
	let backup_path = backup_path.trim();
	if !backup_path.is_empty() && std::path::Path::new(backup_path).exists() {
		std::fs::copy(backup_path, MEMORY_FILE)?;
		println!("Restored memory from backup: {}", backup_path);
	} else {
		// Create empty encrypted memory file
		let memory = Memory::default();
		let key = Key::from_slice(&key_bytes);
		let encrypted = encrypt_memory(&memory, key)?;
		std::fs::write(MEMORY_FILE, &encrypted)?;
		println!("No backup provided. Created new empty memory file.");
	}
	println!("Master key and memory file recreated. You may now access your notes.");
	// Audit log
	let timestamp = chrono::Utc::now();
	use std::io::Write as _;
	let audit_entry = format!("recovery performed at {}\n", timestamp);
	std::fs::OpenOptions::new().create(true).append(true).open("audit.log")?.write_all(audit_entry.as_bytes())?;
	Ok(())
}
use anyhow::{Result, anyhow};
use std::time::Duration;
// Network config file to store onboarding network permission
const NETWORK_CONFIG_FILE: &str = "network.cfg";
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::Aead, KeyInit};
use rand::RngCore;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use base64;

const MASTER_KEY_FILE: &str = "master.key";
const MEMORY_FILE: &str = "memory.bin";
const RECOVERY_FILE: &str = "recovery.json";

#[derive(Serialize, Deserialize, Debug)]
struct RecoveryInfo {
	uuid: Uuid,
	timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Note {
	value: String,
	tags: Vec<String>,
	timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Memory {
	items: HashMap<String, Note>,
	history: Vec<String>, // command history
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct UserProfile {
	name: String,
	language: String,
	allow_network: bool,
}

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 {
	match args[1].as_str() {
			"onboard" => {
				onboard()?;
				println!("Onboarding complete.");
			},
			"network-test" => {
				if !network_allowed()? {
					println!("Network operations are not enabled. Run onboarding and allow network operations.");
					return Ok(());
				}
				network_test()?;
			},
			"recover" => {
				recover_from_file()?;
			},
			"save" => {
				if args.len() < 4 {
					println!("Usage: save <key> <value>");
					return Ok(());
				}
				let key = load_master_key()?;
				let mut memory = load_memory(&key)?;
				save_memory(&mut memory, &key, &args[2], &args[3], &[])?;
			},
			"get" => {
				if args.len() < 3 {
					println!("Usage: get <key>");
					return Ok(());
				}
				let key = load_master_key()?;
				let memory = load_memory(&key)?;
				get_memory(&memory, &args[2])?;
			},
			"show" => {
				let key = load_master_key()?;
				let memory = load_memory(&key)?;
				show_memory(&memory)?;
			},
			_ => {
				println!("Unknown command. Available: onboard, recover, save, get, show");
			}
		}
		return Ok(());
	}
	// Interactive menu fallback
	let master_key = match load_master_key() {
		Ok(key) => Some(key),
		Err(_) => None,
	};
	let memory = match master_key.as_ref() {
		Some(key) => match load_memory(key) {
			Ok(mem) => mem,
			Err(_) => Memory::default(),
		},
		None => Memory::default(),
	};
	let mut memory = memory;
	let mut master_key = master_key;
	loop {
		println!("\nZLang CLI Menu:");
		println!("1) Run onboarding");
		println!("2) Show memory summary");
		println!("3) Save a memory item (key/value/tags)");
		println!("4) Get a memory item (key)");
		println!("5) Search notes");
		println!("6) Show notes by tag");
		println!("7) Undo last operation");
		println!("8) Export memory");
		println!("9) Import memory");
		println!("10) Switch user profile");
		println!("11) Sync memory (network test)");
		println!("12) Exit");
		print!("Select option: ");
		io::stdout().flush()?;
		let mut choice = String::new();
		io::stdin().read_line(&mut choice)?;
		match choice.trim() {
			"1" => {
				onboard()?;
				master_key = Some(load_master_key()?);
				memory = load_memory(master_key.as_ref().unwrap())?;
			},
			"2" => show_memory(&memory)?,
			"3" => {
				let (key, value, tags) = prompt_key_value_tags()?;
				save_memory(&mut memory, master_key.as_ref().ok_or(anyhow!("Run onboarding first"))?, &key, &value, &tags)?;
			},
			"4" => {
				let key = prompt_key()?;
				get_memory(&memory, &key)?;
			},
			"5" => {
				let keyword = prompt_search_keyword()?;
				search_notes(&memory, &keyword)?;
			},
			"6" => {
				let tag = prompt_tag()?;
				show_notes_by_tag(&memory, &tag)?;
			},
			"7" => undo_last(&mut memory)?,
			"8" => export_memory(&memory)?,
			"9" => import_memory(&mut memory)?,
			"10" => switch_user()?,
			"11" => {
				if !network_allowed()? {
					println!("Network operations are not enabled. Run onboarding and allow network operations.");
				} else {
					sync_memory()?;
				}
			},
			"12" => {
				println!("Exiting ZLang CLI.");
				break;
			},
			_ => println!("Invalid option. Try again."),
		}
	}
	Ok(())
}

fn sync_memory() -> Result<()> {
	println!("--- Sync Memory (Network Test) ---");
	let url = "https://httpbin.org/get";
	let client = match ureq::AgentBuilder::new().timeout(std::time::Duration::from_secs(5)).build().get(url)
		.set("User-Agent", "zlang-cli")
		.call() {
		Ok(resp) => resp,
		Err(e) => {
			println!("Network error: {}", e);
			return Ok(());
		}
	};
	let text = match client.into_string() {
		Ok(t) => t,
		Err(e) => {
			println!("Failed to read response: {}", e);
			return Ok(());
		}
	};
	println!("Network response: {}", text);
	Ok(())
	}

fn onboard() -> Result<()> {
	println!("--- Onboarding ---");
	print!("Enter your name: ");
	io::stdout().flush()?;
	let mut name = String::new();
	io::stdin().read_line(&mut name)?;
	print!("Preferred language (en, hi, es): ");
	io::stdout().flush()?;
	let mut lang = String::new();
	io::stdin().read_line(&mut lang)?;
	print!("Allow network operations? (y/n): ");
	io::stdout().flush()?;
	let mut net = String::new();
	io::stdin().read_line(&mut net)?;
	fs::write(NETWORK_CONFIG_FILE, net.trim())?;

	// Generate master key
	let mut rng = ChaCha20Rng::from_entropy();
	let mut key_bytes = [0u8; 32];
	rng.fill_bytes(&mut key_bytes);
	fs::write(MASTER_KEY_FILE, &key_bytes)?;

	// Create empty encrypted memory file
	let memory = Memory::default();
	let key = Key::from_slice(&key_bytes);
	let encrypted = encrypt_memory(&memory, key)?;
	fs::write(MEMORY_FILE, &encrypted)?;

	// Generate recovery file
	let recovery = RecoveryInfo {
		uuid: Uuid::new_v4(),
		timestamp: Utc::now(),
	};
	let recovery_json = serde_json::to_string_pretty(&recovery)?;
	fs::write(RECOVERY_FILE, &recovery_json)?;
	// Audit log
	let audit_entry = format!("onboarded user at {}\n", recovery.timestamp);
	use std::io::Write as _;
	std::fs::OpenOptions::new().create(true).append(true).open("audit.log")?.write_all(audit_entry.as_bytes())?;
	println!("Onboarding complete. Recovery file created: {}", RECOVERY_FILE);
	Ok(())
}

fn show_memory(memory: &Memory) -> Result<()> {
	println!("--- Memory Summary ---");
	for (k, v) in &memory.items {
		println!("{}: {:?}", k, v);
	}
	if memory.items.is_empty() {
		println!("No memory items found.");
	}
	Ok(())
}

fn save_memory(memory: &mut Memory, key: &Key, item_key: &str, item_value: &str, tags: &[String]) -> Result<()> {
    let note = Note {
        value: item_value.to_string(),
        tags: tags.to_vec(),
        timestamp: chrono::Utc::now(),
    };
    memory.items.insert(item_key.to_string(), note);
    memory.history.push(format!("save:{}", item_key));
    let encrypted = encrypt_memory(memory, key)?;
    fs::write(MEMORY_FILE, &encrypted)?;
    println!("Saved {}", item_key);
    Ok(())
}

fn get_memory(memory: &Memory, item_key: &str) -> Result<()> {
    match memory.items.get(item_key) {
        Some(note) => {
            println!("{}: {}", item_key, note.value);
            if !note.tags.is_empty() {
                println!("Tags: {}", note.tags.join(", "));
            }
            println!("Timestamp: {}", note.timestamp);
        },
        None => println!("Key '{}' not found.", item_key),
    }
    Ok(())
}

fn prompt_key_value_tags() -> Result<(String, String, Vec<String>)> {
    print!("Enter key: ");
    io::stdout().flush()?;
    let mut key = String::new();
    io::stdin().read_line(&mut key)?;
    print!("Enter value: ");
    io::stdout().flush()?;
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    print!("Enter tags (comma separated): ");
    io::stdout().flush()?;
    let mut tags = String::new();
    io::stdin().read_line(&mut tags)?;
    let tags: Vec<String> = tags.trim().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    Ok((key.trim().to_string(), value.trim().to_string(), tags))
}

fn prompt_key() -> Result<String> {
	print!("Enter key: ");
	io::stdout().flush()?;
	let mut key = String::new();
	io::stdin().read_line(&mut key)?;
	Ok(key.trim().to_string())
}

fn prompt_search_keyword() -> Result<String> {
    print!("Enter search keyword: ");
    io::stdout().flush()?;
    let mut kw = String::new();
    io::stdin().read_line(&mut kw)?;
    Ok(kw.trim().to_string())
}

fn search_notes(memory: &Memory, keyword: &str) -> Result<()> {
    println!("--- Search Results ---");
    for (k, note) in &memory.items {
        if k.contains(keyword) || note.value.contains(keyword) || note.tags.iter().any(|t| t.contains(keyword)) {
            println!("* {}: {} [tags: {}]", k, note.value, note.tags.join(", "));
        }
    }
    Ok(())
}

fn prompt_tag() -> Result<String> {
    print!("Enter tag: ");
    io::stdout().flush()?;
    let mut tag = String::new();
    io::stdin().read_line(&mut tag)?;
    Ok(tag.trim().to_string())
}

fn show_notes_by_tag(memory: &Memory, tag: &str) -> Result<()> {
    println!("--- Notes with tag '{}' ---", tag);
    for (k, note) in &memory.items {
        if note.tags.iter().any(|t| t == tag) {
            println!("* {}: {}", k, note.value);
        }
    }
    Ok(())
}

fn undo_last(memory: &mut Memory) -> Result<()> {
    if let Some(last) = memory.history.pop() {
        if last.starts_with("save:") {
            let key = &last[5..];
            memory.items.remove(key);
            println!("Undid last save: {}", key);
        }
        // Extend for delete, etc.
        let encrypted = encrypt_memory(memory, &load_master_key()?)?;
        fs::write(MEMORY_FILE, &encrypted)?;
    } else {
        println!("No history to undo.");
    }
    Ok(())
}

fn export_memory(memory: &Memory) -> Result<()> {
    print!("Enter export file path: ");
    io::stdout().flush()?;
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();
    let key = load_master_key()?;
    let encrypted = encrypt_memory(memory, &key)?;
    fs::write(path, &encrypted)?;
    println!("Exported memory to {}", path);
    Ok(())
}

fn import_memory(memory: &mut Memory) -> Result<()> {
    print!("Enter import file path: ");
    io::stdout().flush()?;
    let mut path = String::new();
    io::stdin().read_line(&mut path)?;
    let path = path.trim();
    let key = load_master_key()?;
    let encrypted = fs::read(path)?;
    let imported: Memory = decrypt_memory(&encrypted, &key)?;
    *memory = imported;
    println!("Imported memory from {}", path);
    Ok(())
}

fn switch_user() -> Result<()> {
    print!("Enter user name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    // For demo: switch memory file per user
    let user_memory_file = format!("memory_{}.bin", name);
    if !std::path::Path::new(&user_memory_file).exists() {
        println!("No memory file for user '{}'. Starting fresh.", name);
        fs::write(&user_memory_file, encrypt_memory(&Memory::default(), &load_master_key()?)?)?;
    }
    // Symlink/copy user file to main memory.bin
    fs::copy(&user_memory_file, MEMORY_FILE)?;
    println!("Switched to user '{}'.", name);
    Ok(())
}

fn load_master_key() -> Result<Key> {
	let key_bytes = fs::read(MASTER_KEY_FILE)?;
	if key_bytes.len() != 32 {
		return Err(anyhow!("Invalid master key length"));
	}
	Ok(Key::from_slice(&key_bytes).clone())
}

fn load_memory(key: &Key) -> Result<Memory> {
	if !Path::new(MEMORY_FILE).exists() {
		return Ok(Memory::default());
	}
	let encrypted = fs::read(MEMORY_FILE)?;
	decrypt_memory(&encrypted, key)
}

fn encrypt_memory(memory: &Memory, key: &Key) -> Result<Vec<u8>> {
	let cipher = ChaCha20Poly1305::new(key);
	let mut rng = ChaCha20Rng::from_entropy();
	let mut nonce_bytes = [0u8; 12];
	rng.fill_bytes(&mut nonce_bytes);
	let nonce = Nonce::from_slice(&nonce_bytes);
	let serialized = bincode::serialize(memory)?;
	let ciphertext = cipher.encrypt(nonce, serialized.as_ref()).map_err(anyhow::Error::msg)?;
	// Store nonce + ciphertext
	let mut out = Vec::new();
	out.extend_from_slice(&nonce_bytes);
	out.extend_from_slice(&ciphertext);
	Ok(out)
}

fn decrypt_memory(data: &[u8], key: &Key) -> Result<Memory> {
	if data.len() < 12 {
		return Err(anyhow!("Encrypted data too short"));
	}
	let (nonce_bytes, ciphertext) = data.split_at(12);
	let cipher = ChaCha20Poly1305::new(key);
	let nonce = Nonce::from_slice(nonce_bytes);
	let plaintext = cipher.decrypt(nonce, ciphertext).map_err(anyhow::Error::msg)?;
	let memory: Memory = bincode::deserialize(&plaintext)?;
	Ok(memory)
}
