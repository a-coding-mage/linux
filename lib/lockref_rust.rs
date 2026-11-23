// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for lockref reference counts.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `lockref.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "lockref.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(lockref_get, lockref_get, "", "");
ffi_export::export_symbol!(lockref_get_not_zero, lockref_get_not_zero, "", "");
ffi_export::export_symbol!(lockref_put_return, lockref_put_return, "", "");
ffi_export::export_symbol!(lockref_put_or_lock, lockref_put_or_lock, "", "");
ffi_export::export_symbol!(lockref_mark_dead, lockref_mark_dead, "", "");
ffi_export::export_symbol!(lockref_get_not_dead, lockref_get_not_dead, "", "");
