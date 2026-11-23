// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the out-of-line refcount_t functions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `refcount.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "refcount.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(refcount_warn_saturate, refcount_warn_saturate, "", "");
ffi_export::export_symbol!(refcount_dec_if_one, refcount_dec_if_one, "", "");
ffi_export::export_symbol!(refcount_dec_not_one, refcount_dec_not_one, "", "");
ffi_export::export_symbol!(refcount_dec_and_mutex_lock, refcount_dec_and_mutex_lock, "", "");
ffi_export::export_symbol!(refcount_dec_and_lock, refcount_dec_and_lock, "", "");
ffi_export::export_symbol!(refcount_dec_and_lock_irqsave, refcount_dec_and_lock_irqsave, "", "");
