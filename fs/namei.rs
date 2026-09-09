#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// Faithful source-level preservation of the isolated kernel implementation.
// The referenced declarations and definitions are supplied by the surrounding
// kernel translation units; this artifact intentionally does not invent them.
pub const NAMEI_C_SOURCE: &str = include_str!("namei.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
