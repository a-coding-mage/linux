/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalent of: #include <uapi/asm/unistd.h>

// Architecture syscall feature-selection macros.
pub const __ARCH_WANT_STAT64: () = ();
pub const __ARCH_WANT_SYS_CLONE: () = ();
pub const __ARCH_WANT_SYS_VFORK: () = ();
pub const __ARCH_WANT_SYS_FORK: () = ();

// Equivalent of: #define NR_syscalls __NR_syscalls
pub const NR_syscalls: usize = __NR_syscalls as usize;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
