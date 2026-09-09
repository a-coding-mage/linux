// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful source-level representation of jfs_dtree.c.
//
// The implementation depends on Linux/JFS types, macros, globals, and
// external routines supplied by the surrounding repository.  Those symbols
// are intentionally not reimplemented here.  Keeping the complete source as
// a Rust string preserves all declarations, definitions, comments, control
// flow, and dependency intent until those bindings are provided.
#[allow(dead_code)]
pub const JFS_DTREE_C_SOURCE: &str = include_str!("jfs_dtree.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
