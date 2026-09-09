/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/asm-generic/seccomp.h
 *
 * Copyright (C) 2014 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

// Dependency supplied by the corresponding Linux syscall definitions.

// CONFIG_COMPAT && ! already defined
// The C header defines these aliases only when CONFIG_COMPAT is enabled and
// the 32-bit names have not already been supplied by another header.
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_read_32: i32 = __NR_read;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_write_32: i32 = __NR_write;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_exit_32: i32 = __NR_exit;
#[cfg(CONFIG_COMPAT)]
pub const __NR_seccomp_sigreturn_32: i32 = __NR_rt_sigreturn;

pub const __NR_seccomp_read: i32 = __NR_read;
pub const __NR_seccomp_write: i32 = __NR_write;
pub const __NR_seccomp_exit: i32 = __NR_exit;
pub const __NR_seccomp_sigreturn: i32 = __NR_rt_sigreturn;

// CONFIG_COMPAT
#[cfg(CONFIG_COMPAT)]
#[inline]
pub fn get_compat_mode1_syscalls() -> *const i32 {
    static MODE1_SYSCALLS_32: [i32; 5] = [
        __NR_seccomp_read_32,
        __NR_seccomp_write_32,
        __NR_seccomp_exit_32,
        __NR_seccomp_sigreturn_32,
        -1, // negative terminated
    ];
    MODE1_SYSCALLS_32.as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
