libhash-sha3-sha3
===========
[![Build Status](https://travis-ci.org/Fantom-foundation/libhash-sha3.svg?branch=master)](https://travis-ci.org/Fantom-foundation/libhash-sha3)

libhash-sha3 in Rust.

## RFCs

https://github.com/Fantom-foundation/fantom-rfcs

# Developer guide

Install the latest version of [Rust](https://www.rust-lang.org). We tend to use nightly versions. [CLI tool for installing Rust](https://rustup.rs).

We use [rust-clippy](https://github.com/rust-lang-nursery/rust-clippy) linters to improve code quality.

There are plenty of [IDEs](https://areweideyet.com) and other [Rust development tools to consider](https://github.com/rust-unofficial/awesome-rust#development-tools).

### Description

This crate is an sha3 implementation of the Hashing trait, defined in the libHash repository:
(https://github.com/Fantom-foundation/libhash). This hash will be used for the consensus algorithms,
such as the Directed Acyclic Graph (DAG) algorithm (https://github.com/Fantom-foundation/libconsensus-dag).

This crate provides access to a single struct: Hash. Hash stores a reference vector of 32 8 bit 
unsigned integers. These values are then converted to a hash using the sha3 hashing algorithm. 

### Step-by-step guide
```bash
# Install Rust (nightly)
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
# Install cargo-make (cross-platform feature-rich reimplementation of Make)
$ cargo install --force cargo-make
# Install rustfmt (Rust formatter)
$ rustup component add rustfmt
# Clone this repo
$ git clone https://github.com/Fantom-foundation/libhash-sha3 && cd libhash-sha3
# Run tests
$ cargo test
# Format, build and test
$ cargo make
```
