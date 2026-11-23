// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the generic string and memory routines.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `string.c` translation unit.
//!
//! `no_builtins` is the counterpart of the C unit's `-ffreestanding`: LLVM
//! must not lower these loops into calls to the very routines defined here.
#![no_builtins]

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "string.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(strncasecmp, strncasecmp, "", "");
ffi_export::export_symbol!(strcasecmp, strcasecmp, "", "");
ffi_export::export_symbol!(strcpy, strcpy, "", "");
ffi_export::export_symbol!(sized_strscpy, sized_strscpy, "", "");
ffi_export::export_symbol!(stpcpy, stpcpy, "", "");
ffi_export::export_symbol!(strcat, strcat, "", "");
ffi_export::export_symbol!(strncat, strncat, "", "");
ffi_export::export_symbol!(strlcat, strlcat, "", "");
ffi_export::export_symbol!(strcmp, strcmp, "", "");
ffi_export::export_symbol!(strncmp, strncmp, "", "");
ffi_export::export_symbol!(strchr, strchr, "", "");
ffi_export::export_symbol!(strchrnul, strchrnul, "", "");
ffi_export::export_symbol!(strrchr, strrchr, "", "");
ffi_export::export_symbol!(strnchr, strnchr, "", "");
ffi_export::export_symbol!(strlen, strlen, "", "");
ffi_export::export_symbol!(strnlen, strnlen, "", "");
ffi_export::export_symbol!(strspn, strspn, "", "");
ffi_export::export_symbol!(strcspn, strcspn, "", "");
ffi_export::export_symbol!(strpbrk, strpbrk, "", "");
ffi_export::export_symbol!(strsep, strsep, "", "");
ffi_export::export_symbol!(memcmp, memcmp, "", "");
ffi_export::export_symbol!(bcmp, bcmp, "", "");
ffi_export::export_symbol!(memscan, memscan, "", "");
ffi_export::export_symbol!(strstr, strstr, "", "");
ffi_export::export_symbol!(strnstr, strnstr, "", "");
ffi_export::export_symbol!(memchr, memchr, "", "");
ffi_export::export_symbol!(memchr_inv, memchr_inv, "", "");
