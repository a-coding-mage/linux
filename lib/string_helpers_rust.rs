// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the string formatting and escaping helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `string_helpers.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "string_helpers.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(string_get_size, string_get_size, "", "");
ffi_export::export_symbol!(parse_int_array, parse_int_array, "", "");
ffi_export::export_symbol!(parse_int_array_user, parse_int_array_user, "", "");
ffi_export::export_symbol!(string_unescape, string_unescape, "", "");
ffi_export::export_symbol!(string_escape_mem, string_escape_mem, "", "");
ffi_export::export_symbol!(kstrdup_quotable, kstrdup_quotable, "GPL", "");
ffi_export::export_symbol!(kstrdup_quotable_cmdline, kstrdup_quotable_cmdline, "GPL", "");
ffi_export::export_symbol!(kstrdup_quotable_file, kstrdup_quotable_file, "GPL", "");
ffi_export::export_symbol!(kstrdup_and_replace, kstrdup_and_replace, "GPL", "");
ffi_export::export_symbol!(kasprintf_strarray, kasprintf_strarray, "GPL", "");
ffi_export::export_symbol!(kfree_strarray, kfree_strarray, "GPL", "");
ffi_export::export_symbol!(devm_kasprintf_strarray, devm_kasprintf_strarray, "GPL", "");
ffi_export::export_symbol!(skip_spaces, skip_spaces, "", "");
ffi_export::export_symbol!(strim, strim, "", "");
ffi_export::export_symbol!(sysfs_streq, sysfs_streq, "", "");
ffi_export::export_symbol!(match_string, match_string, "", "");
ffi_export::export_symbol!(__sysfs_match_string, __sysfs_match_string, "", "");
ffi_export::export_symbol!(strreplace, strreplace, "", "");
ffi_export::export_symbol!(memcpy_and_pad, memcpy_and_pad, "", "");
