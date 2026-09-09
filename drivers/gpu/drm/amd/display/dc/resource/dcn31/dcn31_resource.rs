/*
 * Faithful source-level translation boundary for the DCN31 resource
 * implementation.  The surrounding kernel translation supplies the C ABI
 * types, register-list macros, and constructors referenced by this unit.
 *
 * The original implementation is retained as a compile-time source payload
 * so that all declarations, definitions, comments, and conditional intent
 * remain available to the translation unit until those shared dependencies
 * are represented in Rust.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// Original implementation body, preserved verbatim for source-level ABI
/// translation.  This is intentionally not parsed as Rust: the referenced
/// kernel declarations and generated register macros are external dependencies.
pub const DCN31_RESOURCE_C_SOURCE: &str = include_str!("dcn31_resource.c");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
