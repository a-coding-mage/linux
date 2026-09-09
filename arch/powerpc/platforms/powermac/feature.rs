/*
 * Faithful source-level representation of the isolated implementation.
 *
 * The original translation unit depends on the Linux PowerMac kernel's
 * generated bindings and configuration-specific declarations.  Keep the
 * complete implementation available verbatim until those external items are
 * supplied by the surrounding Rust translation unit.
 */
pub const FEATURE_C_SOURCE: &str = include_str!("feature.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
