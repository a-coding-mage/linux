// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the decrement-and-lock helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `dec_and_lock.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "dec_and_lock.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(atomic_dec_and_lock, atomic_dec_and_lock, "", "");
ffi_export::export_symbol!(_atomic_dec_and_lock_irqsave, _atomic_dec_and_lock_irqsave, "", "");
ffi_export::export_symbol!(atomic_dec_and_raw_lock, atomic_dec_and_raw_lock, "", "");
ffi_export::export_symbol!(_atomic_dec_and_raw_lock_irqsave, _atomic_dec_and_raw_lock_irqsave, "", "");
