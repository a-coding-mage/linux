// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the lock-less NULL terminated single linked list.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `llist.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "llist.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(llist_del_first, llist_del_first, "GPL", "");
ffi_export::export_symbol!(llist_del_first_this, llist_del_first_this, "GPL", "");
ffi_export::export_symbol!(llist_reverse_order, llist_reverse_order, "GPL", "");
