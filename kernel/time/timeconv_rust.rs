// SPDX-License-Identifier: LGPL-2.0+
//! Native kernel crate for the calendar-time conversion.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `timeconv.c` translation unit.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;
#[path = "timeconv.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(time64_to_tm, time64_to_tm, "", "");
