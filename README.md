# argon2-async

This crate provides a relatively nice async wrapper for the [`argon2`](https://lib.rs/crates/argon2) crate.

# Usage
## Runtime
This crate is runtime agnostic. It defaults to using the `tokio` runtime when pulled in with default features,
but that can be changed. Support for other runtimes is lower on my priority list, though.
* To use `tokio` as the underlying runtime:
```toml
[dependencies]
argon2-async = "0.1"
```
* To use `async-std` as the underlying runtime:
```toml
[dependencies]
argon2-async = { version = "0.1", default-features = false, features = ["async-std-rt"] }
```
* To use another runtime:
```toml
[dependencies]
argon2-async = { version = "0.1", default-features = false, features = ["any-rt"] }
```

## Code Usage
In your code, be sure to call `argon2_async::set_config` before attempting to use any other methods.
```rust
async fn main() {
    // It is *strongly* recommended to not use this as your default config,
    // as it is insecure.
    let config = argon2_async::Config::default();
    argon2_async::set_config(config).await.expect("setting config failed");
}
```

# Configuration
The default config is very insecure, but is extremely fast, ideal for development environments.
A helper tool is built in to find a production-ready config. To use it, run the following on a
production machine (or one with equivalent hardware):
```shell
git clone https://github.com/tazz4843/argon2-async
cd argon2-async
RUSTFLAGS="-Ctarget-cpu=native" cargo run --release
```
This will test a few config settings one by one, and any with a 👍 beside are good configs for production.