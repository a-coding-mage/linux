#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

// Faithful source-preservation boundary for the isolated translation unit.
// The C implementation is retained verbatim as input until the surrounding
// kernel type and macro environment supplies the corresponding Rust ABI items.
pub const CPUFREQ_C_SOURCE: &str = include_str!("cpufreq.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
