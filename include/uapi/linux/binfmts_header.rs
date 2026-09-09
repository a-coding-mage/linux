/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied externally: PAGE_SIZE.

pub struct pt_regs;

/*
 * These are the maximum length and maximum number of strings passed to the
 * execve() system call.  MAX_ARG_STRLEN is essentially random but serves to
 * prevent the kernel from being unduly impacted by misaddressed pointers.
 * MAX_ARG_STRINGS is chosen to fit in a signed 32-bit integer.
 */
pub const MAX_ARG_STRLEN: usize = PAGE_SIZE * 32;
pub const MAX_ARG_STRINGS: i32 = 0x7fffffff;

/* sizeof(linux_binprm->buf) */
pub const BINPRM_BUF_SIZE: usize = 256;

/* preserve argv0 for the interpreter  */
pub const AT_FLAGS_PRESERVE_ARGV0_BIT: i32 = 0;
pub const AT_FLAGS_PRESERVE_ARGV0: i32 = 1 << AT_FLAGS_PRESERVE_ARGV0_BIT;

/*
 * The interpreter runs transparently: the argument vector and the exe
 * link belong to the binary passed in AT_EXECFD.
 */
pub const AT_FLAGS_TRANSPARENT_INTERP_BIT: i32 = 1;
pub const AT_FLAGS_TRANSPARENT_INTERP: i32 = 1 << AT_FLAGS_TRANSPARENT_INTERP_BIT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
