/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: <asm-generic/int-ll64.h> supplies the C types u16 and u32. */

/*
 * These aren't exported outside the kernel to avoid name space clashes
 */
/* The declarations below are omitted when building for __ASSEMBLER__. */
pub type insn_size_t = u16;
pub type reg_size_t = u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
