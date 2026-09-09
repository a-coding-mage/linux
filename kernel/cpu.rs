/*
 * Faithful low-level translation boundary for the Linux CPU hotplug
 * implementation.  The implementation depends on the kernel declarations
 * supplied by the surrounding translation unit; retain the complete source
 * text verbatim until those declarations are available to express the raw
 * pointer/per-CPU ABI in Rust.
 */
pub const CPU_C_SOURCE: &str = include_str!("cpu.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
