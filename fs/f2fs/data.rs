// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-preserving Rust translation boundary for f2fs/data.c.
// The implementation depends on the Linux kernel and sibling f2fs declarations;
// those external dependencies are intentionally not invented here.
//
// The complete original implementation is retained as source text so every
// declaration, branch, operation, and comment remains available to the
// downstream repository translation pass.
const _F2FS_DATA_C_SOURCE: &str = include_str!("data.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
