# virtchk
Confirms the capability of Hardware-Accelerated Virtualization Technology.

## Introduction
This crate is a simple application which checks if hardware-accelerated virtualization technology (e.g.: Intel VT-x, AMD-V) is available. \
For the sake of minimal DLL dependency and binary size, this crate is designed to be `#![no_std]`.

## Run
This repository will be published to [crates.io](https://crates.io), so you will be able to install with:
```
cargo install virtchk
```

## Build
To compile this project:

Install [rust-lang](https://www.rust-lang.org). \
Execute `cargo build` command.
