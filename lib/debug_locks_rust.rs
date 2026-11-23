// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the global lock-debugging flags.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `debug_locks.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "debug_locks.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(debug_locks, debug_locks, "GPL", "");
ffi_export::export_symbol!(debug_locks_silent, debug_locks_silent, "GPL", "");
ffi_export::export_symbol!(debug_locks_off, debug_locks_off, "GPL", "");
