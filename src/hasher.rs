use crate::{Config, Error, Result};
use argon2::{Argon2, Params, PasswordHasher};
use password_hash::{Salt, SaltString};

async fn get_hasher<'a>() -> Result<Argon2<'a>> {
    let config = crate::config::GLOBAL_CONFIG.get().ok_or(Error::MissingConfig)?.read().await;

    let Config {
        algorithm,
        version,
        secret_key,
        memory_cost,
        iterations,
        parallelism,
        output_length,
    } = *config;
    let params = Params::new(memory_cost, iterations, parallelism, output_length)?;
    Ok(match secret_key {
        Some(key) => Argon2::new_with_secret(key, algorithm, version, params),
        None => Ok(Argon2::new(algorithm, version, params)),
    }?)
}

/// Hash this password with the given config. Return the hashed password as a raw bytes vector.
///
/// # Errors
/// This function errors if any of the following happen:
/// * the given config is invalid
/// * hashing the password fails
/// * communication between threads fails
pub async fn hash_raw(password: impl AsRef<[u8]>) -> crate::Result<Vec<u8>> {
    let hasher = get_hasher().await?;
    let password = password.as_ref().to_owned();

    let res = crate::spawn_task::spawn_task(move |x| {
        let salt_str = SaltString::generate(rand::thread_rng());
        let salt = salt_str.as_salt();
        let mut output = Vec::new();
        let output = hasher
            .hash_password_into(&*password, salt.as_bytes(), &mut output)
            .map(|_| output);
        let _ = x.send(output);
    });

    Ok(res.await??)
}

/// Hash this password with the given config. Return the hashed password as a String.
///
/// # Errors
/// This function errors if any of the following happen:
/// * the given config is invalid
/// * hashing the password fails
/// * communication between threads fails
pub async fn hash(password: impl AsRef<[u8]>) -> crate::Result<String> {
    let hasher = get_hasher().await?;
    let password = password.as_ref().to_owned();

    let res = crate::spawn_task::spawn_task(move |x| {
        let salt_str = SaltString::generate(rand::thread_rng());
        let salt = Salt::from(&salt_str);
        let output = hasher.hash_password(&*password, &salt).map(|x| x.serialize().to_string());
        let _ = x.send(output);
    });

    Ok(res.await??)
}
