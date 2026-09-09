/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Architecture-neutral AT_ values in 0-17, leave some room
 * for more of them, start the x86-specific ones at 32.
 */

#[cfg(target_arch = "x86")]
pub const AT_SYSINFO: usize = 32;

pub const AT_SYSINFO_EHDR: usize = 33;

/* entries in ARCH_DLINFO: */
/*
 * In the kernel, AT_VECTOR_SIZE_ARCH is 3 when IA32 emulation is enabled
 * or when building non-x86-64; otherwise (non-compat x86-64) it is 2.
 * The kernel configuration conditions have no direct file-local Rust
 * equivalent, so the non-compat x86-64 value is retained here.
 */
pub const AT_VECTOR_SIZE_ARCH: usize = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
