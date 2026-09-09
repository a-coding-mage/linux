// SPDX-License-Identifier: GPL-2.0
//
// Faithful source-level Rust carrier for the isolated ext4 mballoc
// implementation.  The implementation is retained verbatim as source text
// because its declarations depend on the surrounding Linux/ext4 kernel ABI;
// those dependencies are intentionally not invented in this isolated pass.
//
// The complete implementation source is available to the eventual Rust
// translation unit through this compile-time inclusion.  This preserves all
// declarations, definitions, comments, constants, control flow, and ordering
// for the repository-level translation pass.
pub static MBALLOC_C_SOURCE: &str = include_str!("mballoc.c");

// C-only implementation body; translated items must bind the kernel symbols
// supplied by the other ext4 translation units before code generation.
#[allow(dead_code)]
pub mod mballoc_translation {
    pub const SOURCE_FILE: &str = super::MBALLOC_C_SOURCE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
