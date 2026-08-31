/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency intent from C header: #include <uapi/asm-generic/mman.h> */

/* MAP_32BIT is undefined on arm, fix it for perf */
pub const MAP_32BIT: i32 = 0;
