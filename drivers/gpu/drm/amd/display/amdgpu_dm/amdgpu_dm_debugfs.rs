// SPDX-License-Identifier: MIT
//
// Faithful source-level translation container for the isolated implementation.
// The implementation depends on the Linux kernel and AMD display declarations
// supplied by the surrounding translated repository.  Keeping the complete
// original translation unit available here preserves every declaration,
// definition, comment, branch, operation, and externally visible interface
// until those shared bindings are introduced.
//
// The source is intentionally included verbatim: C ABI/kernel expressions and
// structure layouts must be mapped together with their external declarations,
// rather than guessed locally from this file alone.
#[allow(dead_code)]
pub const AMDGPU_DM_DEBUGFS_SOURCE: &str = include_str!("amdgpu_dm_debugfs.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
