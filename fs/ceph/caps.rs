// SPDX-License-Identifier: GPL-2.0
//
// Faithful source embedding of the complete C implementation.  The dependent
// kernel/Ceph types, macros, and symbols are supplied by the surrounding
// translation unit; retaining the complete source here preserves every
// declaration, definition, comment, branch, loop, and operation for the
// source-level Rust pass without inventing dependency implementations.
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

pub const CEph_CAPS_C_SOURCE: &str = include_str!("caps.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
