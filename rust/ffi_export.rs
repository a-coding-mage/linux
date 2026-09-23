// SPDX-License-Identifier: GPL-2.0-only
//! Shared native-owner import of the translated kernel export definitions.

#[path = "../include/linux/export_header.rs"]
mod translated;

// Implementation owners and the library metadata bridge use different macros.
#[allow(unused_imports)]
pub(crate) use translated::{export_symbol, export_symbol_linkage_gpl};
