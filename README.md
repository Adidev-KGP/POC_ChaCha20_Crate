# Running the Repository

git clone `https://github.com/Adidev-KGP/POC_ChaCha20_Crate.git`

`cargo build` to build the project

`cargo test` to run all the tests

# Publishing as a separate crate on crate.io

Modify the following in the Cargo.toml file of the root directory:

`name`

`version`

`edition`

`description`

`authors`

After modifying the `Cargo.toml` file run `cargo publish` to publish the new crate on `cargo.io`

# Folder Structure

The folder `src` 3 files :

`block.rs` -> Has all the code to return a `ChaCha20 block`

`chacha20_algo.rs` -> Has all the code to use the chacha20 block function to produce keystreams and hence produce the encrypted data.

`lib.rs` -> It is a default file in rust that says it's a `library crate`. It exports all the code in `block.rs` and `chacha20_algo.rs` so that they can be used in other files as well.

The folder `tests` has 2 files :

`chacha20_block_test.rs` -> Test vector for ChaCha20 block function.

`chacha20_test.rs` -> Test vector for ChaCha20 encryption algorithm.

# The .github/workflows/rust-ci.yml

This workflow runs on every push to the `main` branch and every pull request into the main branch. It installs the stable Rust toolchain, Rustfmt, and Clippy. It then runs Clippy and Rustfmt checks using the stable toolchain, and builds and tests the code with the nightly toolchain.

# Contributions

Make changes and raise PR on a separate branch. IF they pass the workflows with the logic, they will be merged.

# THANKS