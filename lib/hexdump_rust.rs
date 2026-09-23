// SPDX-License-Identifier: GPL-2.0-only
//! Kernel crate for the translated hexadecimal conversion implementation.
//!
//! Kbuild supplies the normal no-std kernel Rust flags. The separate crate
//! name lets the original C object remain selectable without a source-rule
//! ambiguity between adjacent `hexdump.c` and `hexdump.rs` files.

#[path = "hexdump.rs"]
mod implementation;

pub use implementation::*;
