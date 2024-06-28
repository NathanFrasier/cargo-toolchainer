# cargo-toolchainer

TODO Badges

cargo-toolchainer is a simple CLI tool to update the channel in your rust-toolchain.toml file.

## Motivation

Rust's `rust-toolchain.toml` file is used widely to lock down the Rust version for projects, however, often these projects will want to upgrade their rust version _eventually_.
In most cases, this is a manual process, and manual processess are subject to error. What this process needs is to be _triggered_ automatically and have manual oversight.
This tool aims to be a component in the automatic portion of that workflow.

## Installation
Install with `cargo install --locked cargo-toolchainer`. This tool requires at least rustc 1.74.1 to build and install.

## Usage
Once installed, run with `cargo toolchainer update` to automatically detect the channel in your `rust-toolchain.toml` file and update it to the newest version. 

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

