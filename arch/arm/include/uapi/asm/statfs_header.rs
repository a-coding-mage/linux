/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * With EABI there is 4 bytes of padding added to this structure.
 * Let's pack it so the padding goes away to simplify dual ABI support.
 * Note that user space does NOT have to pack this structure.
 */
// C equivalent: #define ARCH_PACK_STATFS64 __attribute__((packed,aligned(4)))
// Rust equivalent when applied to a declaration: #[repr(C, packed(4))]

// Dependency supplied by the corresponding asm-generic/statfs.h translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
