//! An async wrapper around [`argon2`]
//!
//! [`argon2`]: (argon2)

mod config;
mod error;
mod hasher;
mod spawn_task;

pub type Result<T> = std::result::Result<T, Error>;

pub use config::*;
pub use error::Error;
pub use hasher::*;
