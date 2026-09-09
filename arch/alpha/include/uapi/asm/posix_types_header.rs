/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * This file is generally used by user-level software, so you need to
 * be a little careful about namespace pollution etc.  Also, we cannot
 * assume GCC is being used.
 */

pub type __kernel_ino_t = ::core::ffi::c_uint;

pub type __kernel_sigset_t = ::core::ffi::c_ulong; /* at least 32 bits */

/* Dependency equivalent of <asm-generic/posix_types.h>. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
