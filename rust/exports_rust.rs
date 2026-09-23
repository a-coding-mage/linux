// SPDX-License-Identifier: GPL-2.0
//! Native export metadata for the kernel's Rust libraries and C helpers.
//!
//! Kbuild generates the lists from the same ordered symbol tables used by
//! `exports.c`. All exports remain GPL-only. Their real definitions and DWARF
//! versions stay in the defining objects; this crate declares no fictitious
//! Rust types for them and must not generate a second set of version CRCs.
//!
//! Source counterpart: `rust/exports.c`. The distinct basename keeps its
//! original C object selectable without a C/Rust source-rule ambiguity.

#![no_std]

mod ffi_export;

include!(concat!(env!("OBJTREE"), "/rust/exports_core_generated.rs"));
include!(concat!(
    env!("OBJTREE"),
    "/rust/exports_bindings_generated.rs"
));
include!(concat!(
    env!("OBJTREE"),
    "/rust/exports_kernel_generated.rs"
));

#[cfg(not(CONFIG_RUST_INLINE_HELPERS))]
include!(concat!(
    env!("OBJTREE"),
    "/rust/exports_helpers_generated.rs"
));

// Preserve the existing bridge's optional build-assert export and versioning
// policy; the build_error object deliberately skips DWARF version generation.
#[cfg(CONFIG_RUST_BUILD_ASSERT_ALLOW)]
ffi_export::export_symbol_linkage_gpl!(rust_build_error);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
