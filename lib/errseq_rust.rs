// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for errseq_t error sequence counters.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `errseq.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "errseq.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(errseq_set, errseq_set, "", "");
ffi_export::export_symbol!(errseq_sample, errseq_sample, "", "");
ffi_export::export_symbol!(errseq_check, errseq_check, "", "");
ffi_export::export_symbol!(errseq_check_and_advance, errseq_check_and_advance, "", "");
