//! Kookie - A secure, local-first, encrypted secret manager for developers
//!
//! This library provides the core functionality for managing encrypted secrets
//! including passwords, API keys, notes, database credentials, and tokens.

pub mod commands;
pub mod crypto;
pub mod session;
pub mod utils;
pub mod vault;

pub use vault::types::SecretType;
pub use vault::Vault;
