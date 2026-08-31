/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on uapi/asm-generic/mman.h. */

/* MAP_32BIT is undefined on s390, fix it for perf */
pub const MAP_32BIT: i32 = 0;
