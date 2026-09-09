/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * x32 syscall flag bit. Some user programs expect syscall NR macros
 * and __X32_SYSCALL_BIT to have type int, even though syscall numbers
 * are, for practical purposes, unsigned long.
 *
 * Fortunately, expressions like (nr & ~__X32_SYSCALL_BIT) do the right
 * thing regardless.
 */
pub const __X32_SYSCALL_BIT: i32 = 0x40000000;

/*
 * When building outside the kernel, select the ABI-specific syscall header:
 * __i386__ -> unistd_32, __ILP32__ -> unistd_x32, otherwise unistd_64.
 * The corresponding declarations are supplied by the translated ABI headers.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
