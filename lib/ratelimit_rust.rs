// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the generic rate limiter.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `ratelimit.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "ratelimit.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(___ratelimit, ___ratelimit, "", "");
