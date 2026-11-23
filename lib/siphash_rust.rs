// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//! Native kernel crate for SipHash-2-4 and HalfSipHash/SipHash-1-3 PRFs.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `siphash.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "siphash.rs"]
mod implementation;

pub use implementation::*;

#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
ffi_export::export_symbol!(__siphash_aligned, __siphash_aligned, "", "");
ffi_export::export_symbol!(__siphash_unaligned, __siphash_unaligned, "", "");
ffi_export::export_symbol!(siphash_1u64, siphash_1u64, "", "");
ffi_export::export_symbol!(siphash_2u64, siphash_2u64, "", "");
ffi_export::export_symbol!(siphash_3u64, siphash_3u64, "", "");
ffi_export::export_symbol!(siphash_4u64, siphash_4u64, "", "");
ffi_export::export_symbol!(siphash_1u32, siphash_1u32, "", "");
ffi_export::export_symbol!(siphash_3u32, siphash_3u32, "", "");
#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
ffi_export::export_symbol!(__hsiphash_aligned, __hsiphash_aligned, "", "");
ffi_export::export_symbol!(__hsiphash_unaligned, __hsiphash_unaligned, "", "");
ffi_export::export_symbol!(hsiphash_1u32, hsiphash_1u32, "", "");
ffi_export::export_symbol!(hsiphash_2u32, hsiphash_2u32, "", "");
ffi_export::export_symbol!(hsiphash_3u32, hsiphash_3u32, "", "");
ffi_export::export_symbol!(hsiphash_4u32, hsiphash_4u32, "", "");
