# virtchk
Confirms the capability of Hardware-Accelerated Virtualization Technology.

## Introduction
This crate is a simple application which checks if hardware-accelerated virtualization technology (e.g.: Intel VT-x, AMD-V) is available. \
For the sake of minimal DLL dependency and binary size, this crate is designed to be `#![no_std]`.

## Console
This program does not use dynamic allocations, so the number of bytes that `print!`/`println!` macro prints is limited.

## Run
This repository will be published to [crates.io](https://crates.io), so you will be able to install with:
```
cargo install virtchk
```

## Build
To compile this project:

Install [rust-lang](https://www.rust-lang.org). \
Execute `cargo build` command.

## UEFI
This project supports running on baremetal UEFI. You may confirm the processor's capability without interferance of the OS. (e.g.: Microsoft Hyper-V will run the OS in guest mode.)

If you do not need to run `virtchk` in a virtual machine, execute `cargo build --target x86_64-unknown-uefi` will build `virtchk` as an EFI application. \
Put the `virtchk.efi` file to a GPT-partitioned USB flash drive with FAT32 file system in path `efi\boot\bootx64.efi`. Boot to your USB flash drive in order to check virtualization capability.

To run `virtchk` in UEFI of a virtual machine, execute the `build-uefi` batch script which will put `virtchk.efi` into virtual disk images.