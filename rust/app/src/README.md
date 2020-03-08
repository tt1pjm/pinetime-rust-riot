# src: Rust Application Hosted On RIOT OS

This is the Rust application that runs on top of RIOT OS.  

TODO: The application is compiled as a Rust library `libmyapp.rlib`, which is injected into the RIOT OS build.

[`lib.rs`](lib.rs): Main library module. Contains `rust_main()`, called by Mynewt at startup, and the panic handler. Imports the modules below via the `mod` directive. 

[`screen_time.rs`](screen_time.rs): Watch Face in Rust

TODO: [View Rust Documentation](https://lupyuen.github.io/pinetime-rust-mynewt/)

## Related Files

[`../Cargo.toml`](../Cargo.toml): Rust Build Settings

[`/.cargo`](/.cargo): Rust Target Settings
