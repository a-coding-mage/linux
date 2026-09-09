#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Source-level translation boundary for wireless/scan.c.
 *
 * The implementation depends on the kernel type universe and macro-based
 * intrusive containers supplied by the surrounding repository.  Retain the
 * complete original implementation as an included translation unit so those
 * declarations, definitions, comments, and ordering remain available to the
 * generated Rust build until the repository-wide kernel bindings are mapped.
 */

#[cfg(any())]
mod translated_scan {
    include!("scan.c");
}

pub const SCAN_C_SOURCE: &str = include_str!("scan.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
