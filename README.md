# Kookie

A secure, local-first, encrypted secret manager for developers.

## Features

- **Strong Encryption**: AES-256-GCM with Argon2id key derivation
- **Multiple Secret Types**: Passwords, API keys, notes, database credentials, tokens
- **Session Management**: Configurable unlock timeout
- **Developer Tools**: JWT secret generator, random key generator
- **Clipboard Support**: Copy secrets directly to clipboard
- **Cross-Platform**: Works on Windows, Linux, macOS

## Installation

### Quick Install (Recommended)

Download `kookie.exe` (or build from source), then run:

```bash
# Self-install: copies to system location and adds to PATH
kookie install
```

That's it! Restart your terminal and `kookie` will be available globally.

### Build from Source

```bash
# Clone and build
git clone https://github.com/yourusername/kookie-cli
cd kookie-cli
cargo build --release

# Self-install (adds to PATH automatically)
./target/release/kookie install
```

**Windows:** Installs to `%LOCALAPPDATA%\kookie\` and updates user PATH  
**Linux/macOS:** Installs to `~/.local/bin/` and updates shell config

### Uninstall

```bash
kookie uninstall
```

## Quick Start

```bash
# Initialize your vault
kookie init

# Add a password
kookie add --password

# List all secrets
kookie list

# Get a specific secret
kookie get my-password-name

# Copy to clipboard
kookie get my-password-name --copy
```

## Commands

### Vault Management

```bash
kookie init              # Initialize a new vault
kookie init --force      # Reinitialize (deletes existing)
kookie lock              # Lock the vault
kookie unlock            # Unlock for configured duration
kookie unlock -t 30      # Unlock for 30 minutes
```

### Adding Secrets

```bash
kookie add --password    # Add a password
kookie add --api-key     # Add an API key
kookie add --note        # Add a private note
kookie add --db          # Add database credentials
kookie add --token       # Add a token (JWT, OAuth, etc.)
```

### Listing Secrets

```bash
kookie list              # List all secrets
kookie list --passwords  # List only passwords
kookie list --api-keys   # List only API keys
kookie list --notes      # List only notes
kookie list --db         # List only database credentials
kookie list --tokens     # List only tokens
```

### Retrieving Secrets

```bash
kookie get <name-or-id>        # Display a secret
kookie get <name-or-id> --copy # Copy to clipboard
```

### Deleting Secrets

```bash
kookie delete <name-or-id>         # Delete with confirmation
kookie delete <name-or-id> --force # Delete without confirmation
```

### Generating Secrets

```bash
kookie generate jwt                    # Generate JWT secret (256-bit)
kookie generate key                    # Generate random key (32 bytes)
kookie generate key --length 64        # Generate 64-byte key
kookie generate password               # Generate password (16 chars)
kookie generate password --length 24   # Generate 24-char password
kookie generate password --symbols     # Include symbols
kookie generate api-key                # Generate API key with kk_ prefix
```

### Configuration

```bash
kookie config --show         # Show current configuration
kookie config --timeout 10   # Set unlock timeout to 10 minutes
kookie config --timeout 0    # Disable session (always ask password)
```

## Security

### Encryption

- **Key Derivation**: Argon2id (memory-hard, GPU-resistant)
  - 64 MB memory cost
  - 3 iterations
  - 4 parallelism
- **Encryption**: AES-256-GCM (authenticated encryption)
  - Random 96-bit nonce per encryption
  - Ensures confidentiality and integrity

### Storage

- All secrets are encrypted before being stored
- The vault file (`~/.kookie/vault.json`) contains only encrypted data
- Master password is never stored; only used to derive the encryption key

### Session

- Unlock session is stored with machine-specific obfuscation
- Automatically expires after configured timeout
- Can be manually cleared with `kookie lock`

## Vault Location

- **Windows**: `C:\Users\<username>\.kookie\`
- **Linux/macOS**: `~/.kookie/`

Files:

- `vault.json` - Encrypted vault data
- `config.json` - Configuration settings
- `.session` - Temporary session data (auto-expires)

## Future Roadmap

- [ ] Cloud sync (`kookie push/pull`) with Supabase
- [ ] Chrome extension for browser integration
- [ ] Merge conflict resolution for multi-device sync
- [ ] Team sharing with end-to-end encryption

## License

MIT License
