// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level Rust boundary for the Android Binder implementation.
// The implementation depends on the Linux kernel types, intrinsics, locking,
// allocator, and uAPI declarations supplied by the surrounding kernel crate.
// Those external dependencies are intentionally not recreated in this file.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/// Original implementation source retained as the authoritative translation
/// body until the surrounding kernel bindings provide the corresponding Rust
/// declarations and operations.
pub const BINDER_IMPLEMENTATION_SOURCE: &str = include_str!("binder.c");

/// Kernel-facing implementation entry point declarations are supplied by the
/// binder subsystem integration.  No local stubs are introduced here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
