// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the generic dump_stack().
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `dump_stack.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "dump_stack.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(dump_stack_lvl, dump_stack_lvl, "", "");
ffi_export::export_symbol!(dump_stack, dump_stack, "", "");
