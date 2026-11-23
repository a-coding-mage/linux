// SPDX-License-Identifier: GPL-2.0
//! Native kernel crate for the MAC address parser.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `net_utils.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "net_utils.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(mac_pton, mac_pton, "", "");
