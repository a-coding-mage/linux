/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Indicates the presence of extended state information in the memory
 * layout pointed by the fpstate pointer in the ucontext's sigcontext
 * struct (uc_mcontext).
 */
pub const UC_FP_XSTATE: u32 = 0x1;

/* The following constants are available when building for x86_64. */
#[cfg(target_arch = "x86_64")]
pub const UC_SIGCONTEXT_SS: u32 = 0x2;

#[cfg(target_arch = "x86_64")]
pub const UC_STRICT_RESTORE_SS: u32 = 0x4;

/*
 * The declarations from <asm-generic/ucontext.h> are supplied by the
 * corresponding Rust translation of that dependency.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
