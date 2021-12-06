//! An async wrapper around [`argon2`]
//!
//! [`argon2`]: (argon2)

#![deny(missing_docs)]

mod config;
mod error;
mod hasher;
mod spawn_task;
mod verifier;

/// A type helper for every result returned by this crate.
pub type Result<T> = std::result::Result<T, Error>;

pub use config::*;
pub use error::Error;
pub use hasher::*;
pub use verifier::*;
