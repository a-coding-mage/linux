// SPDX-License-Identifier: GPL-2.0-or-later
//! Native owner of the original rbtree C exports.
#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "rbtree.rs"]
/// Complete original provider implementation and its inline headers.
pub mod implementation;
pub use implementation::*;
ffi_export::export_symbol!(__rb_erase_color, __rb_erase_color, "", "");
ffi_export::export_symbol!(rb_insert_color, rb_insert_color, "", "");
ffi_export::export_symbol!(rb_erase, rb_erase, "", "");
ffi_export::export_symbol!(rb_erase_linked, rb_erase_linked, "GPL", "");
ffi_export::export_symbol!(__rb_insert_augmented, __rb_insert_augmented, "", "");
ffi_export::export_symbol!(rb_next, rb_next, "", "");
ffi_export::export_symbol!(rb_prev, rb_prev, "", "");
ffi_export::export_symbol!(rb_replace_node, rb_replace_node, "", "");
ffi_export::export_symbol!(rb_replace_node_rcu, rb_replace_node_rcu, "", "");
ffi_export::export_symbol!(rb_next_postorder, rb_next_postorder, "", "");
ffi_export::export_symbol!(rb_first_postorder, rb_first_postorder, "", "");
