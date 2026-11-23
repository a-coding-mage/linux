// SPDX-License-Identifier: GPL-2.0+
//! Native kernel crate for the cycle-counter based time counter.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `timecounter.c` translation unit.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;
#[path = "timecounter.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(timecounter_init, timecounter_init, "GPL", "");
ffi_export::export_symbol!(timecounter_read, timecounter_read, "GPL", "");
