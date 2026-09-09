/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * We need compat_statfs64 to be packed, because the i386 ABI won't
 * add padding at the end to bring it to a multiple of 8 bytes, but
 * the x86_64 ABI will.
 *
 * C equivalent: __attribute__((packed, aligned(4)))
 */
// ARCH_PACK_COMPAT_STATFS64

/* C dependency: <asm-generic/statfs.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
