# x86_64 Rust OS

A minimal operating system kernel written in Rust, following the [*Writing an OS in Rust*](https://os.phil-opp.com/) tutorial series by Philipp Oppermann.

## Features Added 
- Bootable via BIOS with custom bootloader
- VGA text mode output (`vga_buffer`)
- Serial port logging (`serial`)
- Global Descriptor Table (GDT) setup
- Interrupt Descriptor Table (IDT) with handlers for:
  - Breakpoints
  - Double faults
  - Timer interrupts
  - Keyboard input
- Basic testing framework (`test_runner`)



## Requirements
- Rust nightly (see `rust-toolchain`)
- `bootimage` for building (see tutorial)
- QEMU for emulation

## Building & Running
```sh
cargo build
bootimage build
bootimage run
```

## Testing
Run all tests (requires QEMU):
```sh
cargo test
```

## Resources
- [Original Tutorial](https://os.phil-opp.com/)
- [x86_64 crate docs](https://docs.rs/x86_64/latest/x86_64/)
- [Bootimage crate](https://github.com/rust-osdev/bootimage)


### Notes:
1. **Tutorial Attribution**: All credits to the original tutorial source [*Writing an OS in Rust*](https://os.phil-opp.com/)

