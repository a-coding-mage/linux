// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the kstrto*() string-to-integer conversions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `kstrtox.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "kstrtox.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(kstrtoull, kstrtoull, "", "");
ffi_export::export_symbol!(kstrtoll, kstrtoll, "", "");
ffi_export::export_symbol!(_kstrtoul, _kstrtoul, "", "");
ffi_export::export_symbol!(_kstrtol, _kstrtol, "", "");
ffi_export::export_symbol!(kstrtouint, kstrtouint, "", "");
ffi_export::export_symbol!(kstrtoint, kstrtoint, "", "");
ffi_export::export_symbol!(kstrtou16, kstrtou16, "", "");
ffi_export::export_symbol!(kstrtos16, kstrtos16, "", "");
ffi_export::export_symbol!(kstrtou8, kstrtou8, "", "");
ffi_export::export_symbol!(kstrtos8, kstrtos8, "", "");
ffi_export::export_symbol!(kstrtobool, kstrtobool, "", "");
ffi_export::export_symbol!(kstrtoudec64, kstrtoudec64, "", "");
ffi_export::export_symbol!(kstrtodec64, kstrtodec64, "", "");
ffi_export::export_symbol!(kstrtobool_from_user, kstrtobool_from_user, "", "");
ffi_export::export_symbol!(kstrtoull_from_user, kstrtoull_from_user, "", "");
ffi_export::export_symbol!(kstrtoll_from_user, kstrtoll_from_user, "", "");
ffi_export::export_symbol!(kstrtoul_from_user, kstrtoul_from_user, "", "");
ffi_export::export_symbol!(kstrtol_from_user, kstrtol_from_user, "", "");
ffi_export::export_symbol!(kstrtouint_from_user, kstrtouint_from_user, "", "");
ffi_export::export_symbol!(kstrtoint_from_user, kstrtoint_from_user, "", "");
ffi_export::export_symbol!(kstrtou16_from_user, kstrtou16_from_user, "", "");
ffi_export::export_symbol!(kstrtos16_from_user, kstrtos16_from_user, "", "");
ffi_export::export_symbol!(kstrtou8_from_user, kstrtou8_from_user, "", "");
ffi_export::export_symbol!(kstrtos8_from_user, kstrtos8_from_user, "", "");
