// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the seq_buf sequence buffer.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `seq_buf.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "seq_buf.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(seq_buf_printf, seq_buf_printf, "GPL", "");
ffi_export::export_symbol!(seq_buf_do_printk, seq_buf_do_printk, "GPL", "");
ffi_export::export_symbol!(seq_buf_puts, seq_buf_puts, "GPL", "");
ffi_export::export_symbol!(seq_buf_putc, seq_buf_putc, "GPL", "");
ffi_export::export_symbol!(seq_buf_putmem_hex, seq_buf_putmem_hex, "GPL", "");
