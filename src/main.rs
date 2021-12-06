use argon2::{Algorithm, Version};
use std::time::Instant;

pub const HASH_LEN: u32 = 32;
pub const ITERATIONS: [u32; 7] = [64, 128, 192, 256, 320, 384, 448];
pub const MEMORY_SIZES: [u32; 7] = [256, 512, 1_024, 2_048, 4_096, 8_192, 16_384];
pub const PASSWORD: &str = "P@ssw0rd";
pub const SALT_LEN: u32 = 32;
pub const ALGORITHM: Algorithm = Algorithm::Argon2id;
pub const VERSION: Version = Version::V0x13;

fn main() {
    let secret_key = Some("t9nGEsDxjWtJYdYeExdB6/HU0vg+rT6czv6HSjVjZng=".as_bytes());
    let threads = num_cpus::get();

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("failed to build tokio rt");

    for memory_size in &MEMORY_SIZES {
        for iterations in &ITERATIONS {
            let elapsed = rt.block_on(async {
                let mut config = argon2_async::Config::new();
                config
                    .set_output_length(Some(HASH_LEN as usize))
                    .set_iterations(*iterations)
                    .set_parallelism(threads as u32)
                    .set_memory_cost(*memory_size)
                    .set_algorithm(ALGORITHM)
                    .set_version(VERSION)
                    .set_secret_key(secret_key);
                argon2_async::set_config(config).await;

                let now = Instant::now();
                argon2_async::hash(PASSWORD).await.expect("failed to run hasher");
                now.elapsed()
            });
            let millis =
                elapsed.as_secs() as f32 * 1_000.0 + elapsed.subsec_nanos() as f32 / 1_000_000.0;
            let ok = if millis > 300.0 && millis < 500.0 {
                "👍"
            } else {
                "💩"
            };
            println!(
                "{} threads {}, memory_size: {}, iterations: {}, milliseconds: {:.0}",
                ok, threads, memory_size, iterations, millis,
            );
        }
    }
}
