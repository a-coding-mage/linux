/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// The C header guard `_M68K_PARAM_H` has no direct Rust equivalent.

// Corresponds to the C `__uClinux__` build-time condition.
#[cfg(feature = "__uClinux__")]
pub const EXEC_PAGESIZE: u32 = 4096;

#[cfg(not(feature = "__uClinux__"))]
pub const EXEC_PAGESIZE: u32 = 8192;

// Dependency corresponding to `<asm-generic/param.h>`; its declarations are
// supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
