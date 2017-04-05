[![license](https://img.shields.io/github/license/lyndir/masterpassword.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![Build Status](https://travis-ci.org/lispyclouds/mpw-rs.svg?branch=master)](https://travis-ci.org/lispyclouds/mpw-rs)

# [Master Password •••|](http://masterpasswordapp.com)

This is the Rust version of the original found [here](https://github.com/Lyndir/MasterPassword).

## Requirements
- [Rust](https://www.rust-lang.org/en-US/install.html) 1.15+

## Building, testing and running
- `cargo test --release`
- `cargo run --release`

## Benchmarking
- `cargo run --release -- --benchmark`

## "Next" features
To use the Argon2 based KDF instead of Scrypt:
- `cargo run --release -- -a next`
- **This should not be used for production** as the Argon2 params are experimental.
- Uses `Argon2i(pass=1, lanes=4, mem=128MB)` as KDF instead of Scrypt.
- **This is only available in the Rust version**.
