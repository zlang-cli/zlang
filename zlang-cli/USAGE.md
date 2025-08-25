# ZLang CLI Onboarding & Recovery Workflow

## Onboarding
1. Run the CLI and select option `1) Run onboarding`.
2. Enter your name, preferred language, and network permission when prompted.
3. The CLI will generate:
   - `master.key` (your encryption key)
   - `memory.bin` (your encrypted memory)
   - `recovery.json` (for account recovery)
   - `network.cfg` (if network is enabled)
4. An audit log entry is created for onboarding.

## Saving & Retrieving Memory
- Use the interactive menu or commands:
  - `save <key> <value>`: Save a memory item.
  - `get <key>`: Retrieve a memory item.
  - `show`: Display all memory items.

## Recovery
1. If you lose access to your memory, use the `recovery.json` file.
2. Run the CLI and select option `recover` or use `./zlang-cli recover`.
3. Follow prompts to restore your account and memory.
4. An audit log entry is created for recovery.

## Multi-User & Profiles
- Switch profiles using the interactive menu (`10) Switch user profile`).
- Each user has separate encrypted memory and recovery files.

## Security Notes
- Never share your `master.key` or `recovery.json`.
- All files are encrypted; only your key can decrypt your memory.
- Audit logs do not contain sensitive data.

---
For more details, see `README.md` or run the CLI and explore the menu options.
