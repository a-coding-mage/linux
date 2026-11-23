// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the allocating printf helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `kasprintf.c` translation unit.
// link_llvm_intrinsics (llvm.returnaddress) is admitted per object.
#![allow(internal_features)]

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "kasprintf.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(kvasprintf, kvasprintf, "", "");
ffi_export::export_symbol!(kvasprintf_const, kvasprintf_const, "", "");
ffi_export::export_symbol!(kasprintf, kasprintf, "", "");
