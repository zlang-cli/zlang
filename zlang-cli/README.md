# ZLang CLI

A secure, encrypted memory storage CLI for personal and multi-user use.

## Features
- Encrypted memory storage using ChaCha20Poly1305
- Onboarding, recovery, and audit logging
- Search, tagging, multi-user support
- Undo/history, export/import
- Network sync (optional)

## Installation
1. Download the binary for your platform (`zlang-cli.exe` for Windows, `zlang-cli` for Linux/macOS).
2. Place the binary in your desired directory.
3. (Optional) Copy `network.cfg` for network features.

## Usage
Run the CLI:
```sh
./zlang-cli.exe   # Windows
./zlang-cli       # Linux/macOS
```

### Interactive Menu
You will see:
```
ZLang CLI Menu:
1) Run onboarding
2) Show memory summary
3) Save a memory item (key/value/tags)
4) Get a memory item (key)
5) Search notes
6) Show notes by tag
7) Undo last operation
8) Export memory
9) Import memory
10) Switch user profile
11) Sync memory (network test)
12) Exit
```
Select an option by entering its number.

### Command-Line Usage
You can also run commands directly:
```sh
./zlang-cli onboard
./zlang-cli save <key> <value>
./zlang-cli get <key>
./zlang-cli show
./zlang-cli recover
```

## Security
- Master key is stored as a binary file (`master.key`), never in plaintext.
- All memory and recovery files are encrypted.
- No sensitive info is printed in logs or errors.

## License
MIT OR Apache-2.0

---
See `USAGE.md` for onboarding and recovery workflow details.
