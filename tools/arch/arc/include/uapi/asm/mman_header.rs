/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Includes <uapi/asm-generic/mman.h>. */

/* MAP_32BIT is undefined on arc, fix it for perf */
pub const MAP_32BIT: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
