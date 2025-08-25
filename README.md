# ZLang Project

## Overview

**ZLang** is a project consisting of two main tools:
- **ZLang CLI:** A secure, encrypted memory storage tool for personal notes, multi-user support, and audit logging.
- **Z Language Tooling:** A command-line interface for running, compiling, and interacting with Z Language scripts.

This README covers both components, with clear sections to help you get started depending on your needs.

---

## ZLang CLI: Secure Encrypted Memory

### Features

- Encrypted memory storage (ChaCha20Poly1305)
- Onboarding, recovery, and audit logging
- Search and tagging
- Multi-user support
- Undo/history, export/import
- Optional network sync

### Quickstart

#### Installation

```sh
# Download binary for your platform or build from source:
git clone https://github.com/zlang-cli/zlang.git
cd zlang
cargo build --release     # Rust
npm install && npm run build  # Node.js
go build -o zlang .       # Go
```
Move the binary to your desired location. (Optional: Copy `network.cfg` for network features.)

#### Running the CLI

```sh
./zlang-cli.exe   # Windows
./zlang-cli       # Linux/macOS
```

#### Interactive Menu

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
Type the menu number to select an option.

#### Command-Line Usage

```sh
./zlang-cli onboard
./zlang-cli save <key> <value>
./zlang-cli get <key>
./zlang-cli show
./zlang-cli recover
```

#### Onboarding Workflow (Step-by-Step)

```plaintext
+---------------------+
|   Run Onboarding    |
+---------------------+
         |
         v
+------------------------------------+
| Enter your name, language, network |
+------------------------------------+
         |
         v
+-----------------------------+
| master.key, memory.bin,     |
| recovery.json generated     |
+-----------------------------+
         |
         v
+-------------------+
| Ready to use CLI! |
+-------------------+
```

#### Recovery

1. If you lose access, use `recovery.json`.
2. Run `./zlang-cli recover` or select menu option.
3. Follow prompts to restore your account/memory.

#### Multi-User & Profiles

- Switch profiles with menu (`10) Switch user profile`)
- Each user has separate encrypted memory/recovery files.

#### Security Notes

- Master key is never stored in plaintext
- All files are encrypted
- Audit logs do not contain sensitive data

---

## Z Language Tooling

### Features

- Run Z Language scripts from the command line
- Compile Z Language files
- Interactive REPL
- Lint and format code

### Usage

```sh
./zlang run script.zl
./zlang compile source.zl
./zlang repl
./zlang --help
./zlang --version
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT OR Apache-2.0  
See [LICENSE](LICENSE) for details.

---

For further details, see `USAGE.md` for onboarding and recovery workflow specifics.
