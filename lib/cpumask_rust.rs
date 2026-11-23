// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the out-of-line cpumask selection helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `cpumask.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "cpumask.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(cpumask_local_spread, cpumask_local_spread, "", "");
ffi_export::export_symbol!(cpumask_any_and_distribute, cpumask_any_and_distribute, "", "");
ffi_export::export_symbol!(cpumask_any_distribute, cpumask_any_distribute, "", "");
