use std::fmt::{Debug, Display, Formatter};
use tokio::task::JoinError;

#[derive(Debug)]
#[non_exhaustive]
/// All possible errors that can happen.
pub enum Error {
    /// An error was encountered while waiting for a background thread to complete.
    Join(JoinError),
    /// The underlying Argon2 hashing implementation threw an error.
    Argon(argon2::Error),
    /// The password string handling library threw an error
    PasswordHash(password_hash::Error),
    /// The global configuration has not been set.
    MissingConfig,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("error while hashing: ")?;
        match self {
            Error::Join(e) => write!(f, "tokio background thread errored: {}", e),
            Error::Argon(e) => write!(f, "error in argon2 hashing algorithm: {}", e),
            Error::PasswordHash(e) => write!(f, "error in password handling lib: {}", e),
            Error::MissingConfig => f.write_str("global configuration has not been set"),
        }
    }
}

impl std::error::Error for Error {}

impl From<JoinError> for Error {
    fn from(e: JoinError) -> Self {
        Self::Join(e)
    }
}

impl From<argon2::Error> for Error {
    fn from(e: argon2::Error) -> Self {
        Self::Argon(e)
    }
}

impl From<password_hash::Error> for Error {
    fn from(e: password_hash::Error) -> Self {
        Self::PasswordHash(e)
    }
}
