#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * Direct source-preserving translation of btrfs/ioctl.c.
 *
 * The implementation depends on the Linux kernel and the surrounding Btrfs
 * translation units for its types, constants, macros, and external symbols.
 * Those dependencies are intentionally left unresolved here, as in the
 * source file.  The complete original implementation is retained verbatim
 * below so that its declarations, control flow, comments, and ABI intent are
 * preserved for the generated kernel bindings.
 */
pub const BTRFS_IOCTL_C_SOURCE: &str = include_str!("ioctl.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
