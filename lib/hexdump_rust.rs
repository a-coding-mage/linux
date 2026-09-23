// SPDX-License-Identifier: GPL-2.0-only
//! Kernel crate for the translated hexadecimal conversion implementation.
//!
//! Kbuild supplies the normal no-std kernel Rust flags. The separate crate
//! name lets the original C object remain selectable without a source-rule
//! ambiguity between adjacent `hexdump.c` and `hexdump.rs` files.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "hexdump.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(hex_asc, HEX_ASC, "", "");
ffi_export::export_symbol!(hex_asc_upper, HEX_ASC_UPPER, "", "");
ffi_export::export_symbol!(hex_to_bin, hex_to_bin, "", "");
ffi_export::export_symbol!(hex2bin, hex2bin, "", "");
ffi_export::export_symbol!(bin2hex, bin2hex, "", "");
ffi_export::export_symbol!(hex_dump_to_buffer, hex_dump_to_buffer, "", "");
#[cfg(CONFIG_PRINTK)]
ffi_export::export_symbol!(print_hex_dump, print_hex_dump, "", "");
