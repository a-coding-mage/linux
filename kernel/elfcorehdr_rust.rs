// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the crash-kernel ELF core header location.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `elfcorehdr.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "elfcorehdr.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(elfcorehdr_addr, elfcorehdr_addr, "GPL", "");
