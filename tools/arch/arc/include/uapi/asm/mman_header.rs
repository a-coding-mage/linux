/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Includes <uapi/asm-generic/mman.h>. */

/* MAP_32BIT is undefined on arc, fix it for perf */
pub const MAP_32BIT: u32 = 0;
