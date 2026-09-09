#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(improper_ctypes, unused_variables, unused_mut, unused_unsafe)]

// Faithful source record: the implementation depends on the NTFS kernel
// declarations and macros supplied by the surrounding translation unit.
pub const FRECORD_C_SOURCE: &str = include_str!("frecord.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
