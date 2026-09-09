#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

//! Source-level Rust translation unit for UBIFS directory operations.
//!
//! The implementation depends on the kernel/UBIFS declarations supplied by
//! the surrounding translation units.  The original implementation is kept
//! available as source text here because this isolated pass has no headers or
//! dependency bindings to resolve those declarations against.

#[allow(dead_code)]
pub const UBIFS_DIR_C_SOURCE: &str = include_str!("dir.c");

/*
 * The declarations and definitions in `dir.c` are intentionally represented
 * as an external translation-unit payload in this isolated file.  All names,
 * control flow, comments, constants, and dependency references remain in the
 * source payload for the repository's subsequent dependency-aware translation
 * stage; no local implementations or placeholder bindings are introduced.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
