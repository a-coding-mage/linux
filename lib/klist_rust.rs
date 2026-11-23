// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for klists.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `klist.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "klist.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(klist_init, klist_init, "GPL", "");
ffi_export::export_symbol!(klist_add_head, klist_add_head, "GPL", "");
ffi_export::export_symbol!(klist_add_tail, klist_add_tail, "GPL", "");
ffi_export::export_symbol!(klist_add_behind, klist_add_behind, "GPL", "");
ffi_export::export_symbol!(klist_add_before, klist_add_before, "GPL", "");
ffi_export::export_symbol!(klist_del, klist_del, "GPL", "");
ffi_export::export_symbol!(klist_remove, klist_remove, "GPL", "");
ffi_export::export_symbol!(klist_node_attached, klist_node_attached, "GPL", "");
ffi_export::export_symbol!(klist_iter_init_node, klist_iter_init_node, "GPL", "");
ffi_export::export_symbol!(klist_iter_init, klist_iter_init, "GPL", "");
ffi_export::export_symbol!(klist_iter_exit, klist_iter_exit, "GPL", "");
ffi_export::export_symbol!(klist_prev, klist_prev, "GPL", "");
ffi_export::export_symbol!(klist_next, klist_next, "GPL", "");
