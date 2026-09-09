/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The original header selects MODULE_PROC_FAMILY from the build-time
 * CONFIG_CPU_LITTLE_ENDIAN, CONFIG_CPU_SH2, CONFIG_CPU_SH3, and
 * CONFIG_CPU_SH4 preprocessor symbols.  These Rust cfg predicates preserve
 * that selection intent.
 */
#[cfg(all(feature = "CONFIG_CPU_LITTLE_ENDIAN", feature = "CONFIG_CPU_SH2"))]
pub const MODULE_PROC_FAMILY: &str = "SH2LE ";

#[cfg(all(feature = "CONFIG_CPU_LITTLE_ENDIAN", feature = "CONFIG_CPU_SH3"))]
pub const MODULE_PROC_FAMILY: &str = "SH3LE ";

#[cfg(all(feature = "CONFIG_CPU_LITTLE_ENDIAN", feature = "CONFIG_CPU_SH4"))]
pub const MODULE_PROC_FAMILY: &str = "SH4LE ";

#[cfg(all(not(feature = "CONFIG_CPU_LITTLE_ENDIAN"), feature = "CONFIG_CPU_SH2"))]
pub const MODULE_PROC_FAMILY: &str = "SH2BE ";

#[cfg(all(not(feature = "CONFIG_CPU_LITTLE_ENDIAN"), feature = "CONFIG_CPU_SH3"))]
pub const MODULE_PROC_FAMILY: &str = "SH3BE ";

#[cfg(all(not(feature = "CONFIG_CPU_LITTLE_ENDIAN"), feature = "CONFIG_CPU_SH4"))]
pub const MODULE_PROC_FAMILY: &str = "SH4BE ";

/* MODULE_ARCH_VERMAGIC is an alias of MODULE_PROC_FAMILY. */
pub const MODULE_ARCH_VERMAGIC: &str = MODULE_PROC_FAMILY;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
