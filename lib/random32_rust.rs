// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the Tausworthe pseudo-random generator helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `random32.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "random32.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(prandom_u32_state, prandom_u32_state, "", "");
ffi_export::export_symbol!(prandom_bytes_state, prandom_bytes_state, "", "");
#[cfg(any(CONFIG_KUNIT, CONFIG_KUNIT = "m"))]
ffi_export::export_symbol!(prandom_warmup, prandom_warmup, "", "EXPORTED_FOR_KUNIT_TESTING");
ffi_export::export_symbol!(prandom_seed_full_state, prandom_seed_full_state, "", "");
