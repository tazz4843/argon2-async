use std::convert::identity;
use argon2::password_hash::Error;
use password_hash::PasswordVerifier;
use crate::Result;

/// Verify a password with the given hash.
///
/// # Returns
/// Returns a `bool`, true if the password is valid, false if not valid.
///
/// # Errors
/// This function errors if any of the following happen:
/// * the given config is invalid
/// * verifying the password fails
/// * communication between threads fails
pub async fn verify<'a>(password: String, hash: String) -> Result<bool> {
    let hasher = crate::get_hasher().await?;
    let res = crate::spawn_task::spawn_task(move |tx| {
        let res = password_hash::PasswordHash::try_from(hash.as_ref()).map(|hash| hasher.verify_password(password.as_bytes(), &hash)).and_then(identity);
        let _ = tx.send(res);
    }).await?;
    match res {
        Ok(()) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(e) => Err(e.into())
    }
}