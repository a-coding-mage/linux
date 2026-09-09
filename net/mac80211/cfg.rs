// SPDX-License-Identifier: GPL-2.0-only
//
// Source-level Rust representation of mac80211/cfg.c.
//
// The implementation depends on the Linux kernel types, macros, globals, and
// helper functions declared by the surrounding mac80211 translation unit.
// Keep the complete isolated source available to the eventual integration
// layer so those external dependencies can be mapped without inventing local
// stubs or changing their ABI.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

/// Complete implementation source retained for the kernel-ABI translation
/// layer.  `cfg.c` is intentionally the only dependency of this isolated
/// artifact; includes and symbols from the kernel are supplied by other files.
pub const CFG_C_SOURCE: &str = include_str!("cfg.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
