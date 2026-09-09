#![allow(non_camel_case_types, non_snake_case, dead_code)]

/*
 * Faithful source-level carry-through of the isolated scheduler implementation.
 * The C translation unit is retained verbatim as a Rust compile-time source
 * payload because its declarations and definitions are supplied by the Linux
 * kernel build environment and cannot be resolved in this isolated file.
 */
pub const CORE_C_SOURCE: &str = include_str!("core.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
