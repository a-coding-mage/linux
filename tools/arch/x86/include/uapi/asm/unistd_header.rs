/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * x32 syscall flag bit.  Some user programs expect syscall NR macros
 * and __X32_SYSCALL_BIT to have type int, even though syscall numbers
 * are, for practical purposes, unsigned long.
 *
 * Fortunately, expressions like (nr & ~__X32_SYSCALL_BIT) do the right
 * thing regardless.
 */
pub const __X32_SYSCALL_BIT: i32 = 0x40000000;

/*
 * C dependency routing preserved from:
 *
 * #ifndef __KERNEL__
 * # ifdef __i386__
 * #  include <asm/unistd_32.h>
 * # elif defined(__ILP32__)
 * #  include <asm/unistd_x32.h>
 * # else
 * #  include <asm/unistd_64.h>
 * # endif
 * #endif
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
