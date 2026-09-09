// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful source-preserving Rust translation boundary for memcontrol.c.
// The implementation is retained verbatim as an embedded source artifact so
// that all declarations, definitions, comments, constants, control flow, and
// external-kernel interfaces remain available without inventing dependencies
// absent from this isolated translation unit.
//
// A direct executable Rust lowering requires the Linux kernel type and macro
// environment supplied by the other repository files.
pub const MEMCONTROL_C_SOURCE: &str = include_str!("memcontrol.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
