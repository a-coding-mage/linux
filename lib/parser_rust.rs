// SPDX-License-Identifier: GPL-2.0-only
//! Native owner of the nine parser C entry points and unrestricted exports.
#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "parser.rs"]
mod implementation;
pub use implementation::*;

ffi_export::export_symbol!(match_token, match_token, "", "");
ffi_export::export_symbol!(match_int, match_int, "", "");
ffi_export::export_symbol!(match_uint, match_uint, "", "");
ffi_export::export_symbol!(match_u64, match_u64, "", "");
ffi_export::export_symbol!(match_octal, match_octal, "", "");
ffi_export::export_symbol!(match_hex, match_hex, "", "");
ffi_export::export_symbol!(match_wildcard, match_wildcard, "", "");
ffi_export::export_symbol!(match_strlcpy, match_strlcpy, "", "");
ffi_export::export_symbol!(match_strdup, match_strdup, "", "");
