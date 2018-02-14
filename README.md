[![license](https://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![Build Status](https://travis-ci.org/rust-hyderabad/mpw-rs.svg?branch=master)](https://travis-ci.org/rust-hyderabad/mpw-rs)
[![Crates.io](https://meritbadge.herokuapp.com/mpw-rs)](https://crates.io/crates/mpw-rs)

# [Master Password •••|](http://masterpasswordapp.com)

This is the Rust version of the original found [here](https://github.com/Lyndir/MasterPassword).
This can be used as a **drop-in replacement for the reference C version**, offering greater runtime safety and memory leaks protection with at-par performance.

<a href="url"><img src="https://user-images.githubusercontent.com/2767425/31902097-4979d340-b841-11e7-8655-2d24515dbccf.png" align="center" height="128" width="128" ></a>

## Requirements
- [Rust](https://www.rust-lang.org/en-US/install.html) 1.15+

## Building, testing and running
- `cargo test --release`
- `cargo run --release`

## Benchmarking
- `cargo run --release -- --benchmark`

## Installing via Cargo
- Make sure **Rust 1.15+** is installed.
- Run `cargo install mpw-rs`
- It will be available on PATH as `mpw-rs` for the current user.

## "Next" features
To use the Argon2 based KDF instead of Scrypt:
- `cargo run --release -- -a next`
- **This should not be used for production** as the Argon2 params are experimental.
- Uses `Argon2i(pass=1, lanes=4, mem=128MB)` as KDF instead of Scrypt.
- **This is only available in the Rust version**.
