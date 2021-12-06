use argon2::{Algorithm, Version};
use once_cell::sync::OnceCell;

pub(crate) static GLOBAL_CONFIG: OnceCell<tokio::sync::RwLock<Config>> = OnceCell::new();

/// Set the global config.
pub async fn set_config(config: Config<'static>) {
    match GLOBAL_CONFIG.get() {
        Some(cfg) => {
            let mut cfg = cfg.write().await;
            *cfg = config;
        }
        None => assert!(GLOBAL_CONFIG.set(tokio::sync::RwLock::new(config)).is_ok())
    }
}

pub struct Config<'k> {
    pub algorithm: Algorithm,
    pub version: Version,
    pub secret_key: Option<&'k [u8]>,
    pub memory_cost: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub output_length: Option<usize>,
}

impl<'k> Default for Config<'k> {
    /// Create a new default config. This is good for basic purposes,
    /// but **SHOULD NOT** be used in a production environment.
    ///
    /// * Algorithm: Argon2id
    /// * Version: 19 (0x13)
    /// * Secret key: None,
    /// * Memory size: 4,096 kilobytes
    /// * Iterations: 3
    /// * Parallelism: 1
    /// * Output length: Some(32)
    #[inline]
    fn default() -> Self {
        Self {
            algorithm: Algorithm::Argon2id,
            version: Version::V0x13,
            secret_key: None,
            memory_cost: 512,
            iterations: 3,
            parallelism: 1,
            output_length: Some(32),
        }
    }
}

impl<'k> Config<'k> {
    /// Create a new config. This is an insecure config and **should not be used in production**!
    #[inline]
    pub fn new_insecure() -> Self {
        Self::default()
    }

    /// Create a new config. This config is somewhat more secure than `new_insecure()` but a secret key should still be set with the `set_secret_key()` function.
    #[inline]
    pub fn new() -> Self {
        Self {
            memory_cost: 8192,
            iterations: 200,
            parallelism: num_cpus::get_physical() as u32,
            ..Self::default()
        }
    }
}

impl<'k> Config<'k> {
    /// Set the hashing algorithm in use.
    ///
    /// The default (Argon2id) should be fine for most uses.
    ///
    /// According to the [latest (as of 5/18) Argon2 RFC](https://tools.ietf.org/html/draft-irtf-cfrg-argon2-03) ...
    /// "Argon2 has one primary variant: Argon2id, and two supplementary
    /// variants: Argon2d and Argon2i. Argon2d uses data-dependent memory
    /// access, which makes it suitable for ... applications with no threats from
    /// side-channel timing attacks. Argon2i uses data-independent memory access,
    /// which is preferred for password hashing and password-based key derivation.
    /// Argon2id works as Argon2i for the first half of the first iteration over the memory,
    /// and as Argon2d for the rest, thus providing both side-channel attack
    /// protection and brute-force cost savings due to time-memory tradeoffs."
    #[inline]
    pub fn set_algorithm(&mut self, algorithm: Algorithm) -> &mut Self {
        self.algorithm = algorithm;
        self
    }

    /// Set the version of `argon2` to use.
    ///
    /// The default (0x13 or v19) should be fine for most uses.
    #[inline]
    pub fn set_version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    /// Set the secret key to use. This is **strongly** recommended in a production environment.
    #[inline]
    pub fn set_secret_key(&mut self, secret_key: Option<&'k [u8]>) -> &mut Self {
        self.secret_key = secret_key;
        self
    }

    /// Set the memory cost in kilobytes.
    ///
    /// Default: 512 for insecure, and 4,096 for secure.
    ///
    /// Argon2 has a notion of "memory size" or "memory cost" (in kilobytes). All else equal
    /// and generally speaking, the greater the memory cost, the longer it takes to perform
    /// the hash and the more secure the resulting hash. More memory cost basically means
    /// more memory used. This and "iterations" are, generally speaking, the two parameters
    /// to adjust in order to increase or decrease the security of your hash. If you're going
    /// to use this crate in production, you should probably tweak this parameter (and the
    /// iterations parameter) in order to increase the time it takes to hash to the maximum
    /// you can reasonably allow for your use-case (e.g. to probably about 300-500 milliseconds
    /// for the use-case of hashing user passwords for a website).
    #[inline]
    pub fn set_memory_cost(&mut self, memory_cost: u32) -> &mut Self {
        self.memory_cost = memory_cost;
        self
    }

    /// Set the number of iterations to use.
    ///
    /// Default: 3 for insecure, and 200 for secure.
    ///
    /// Argon2 has a notion of "iterations" or "time cost". All else equal and generally
    /// speaking, the greater the number of iterations, the longer it takes to perform the
    /// hash and the more secure the resulting hash. More iterations basically means more
    /// CPU load. This and "memory cost" are the two primary parameters to adjust in order
    /// to increase or decrease the security of your hash. If you're going to use this
    /// crate in production, you should probably tweak this parameter (and the memory cost
    /// parameter) in order to increase the time it takes to hash to the maximum you can
    /// reasonably allow for your use-case (e.g. to probably about 300-500 milliseconds for
    /// the use-case of hashing user passwords for a website).
    #[inline]
    pub fn set_iterations(&mut self, iterations: u32) -> &mut Self {
        self.iterations = iterations;
        self
    }

    /// Set the parallelism of the algorithm.
    ///
    /// Default: 1 for insecure, and the number of physical CPU cores on the host for secure.
    ///
    /// Argon2 can break up its work into one or more "lanes" during some parts of the
    /// hashing algorithm. If you configure it with multiple lanes, the hashing algorithm
    /// will perform its work in parallel in some parts, potentially speeding up the time
    /// it takes to produce a hash without diminishing the security of the result.
    #[inline]
    pub fn set_parallelism(&mut self, parallelism: u32) -> &mut Self {
        self.parallelism = parallelism;
        self
    }

    /// Set the output length of the algorithm in bytes.
    ///
    /// Default: 32
    #[inline]
    pub fn set_output_length(&mut self, output_length: Option<usize>) -> &mut Self {
        self.output_length = output_length;
        self
    }
}
