// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the DO_ONCE() slow paths.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `once.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "once.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(__do_once_start, __do_once_start, "", "");
ffi_export::export_symbol!(__do_once_done, __do_once_done, "", "");
ffi_export::export_symbol!(__do_once_sleepable_start, __do_once_sleepable_start, "", "");
ffi_export::export_symbol!(__do_once_sleepable_done, __do_once_sleepable_done, "", "");
