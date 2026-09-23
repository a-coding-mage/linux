// SPDX-License-Identifier: GPL-2.0
//! Safe, allocation-free binary-coded decimal conversions.
//!
//! These pure helpers share their implementation with the translated C ABI,
//! but define no unmangled symbols and call no foreign functions. Independent
//! Rust modules can use this API with either the C or `CONFIG_RUST_BCD` selection.
//! Every helper is usable in a constant expression.
//!
//! The C header selects different arithmetic for constant and runtime arguments.
//! Rust callers choose explicitly: [`bcd2bin`] and [`bin2bcd`] match the exported
//! runtime functions, while [`const_bcd2bin`] and [`const_bin2bcd`] match the
//! full-width C constant expressions for unsigned 32-bit arguments. In
//! particular, `bin2bcd(1024)` is `0xfa`, but `const_bin2bcd(1024)` is `0x664`.
//! Conversion does not reject invalid digits; use [`bcd_is_valid`] when needed.

#[path = "../../include/linux/bcd_header.rs"]
mod conversion;

pub use conversion::*;
