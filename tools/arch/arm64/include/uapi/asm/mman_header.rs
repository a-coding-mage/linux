/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Rust translation of <uapi/asm-generic/mman.h> dependency. */

/* MAP_32BIT is undefined on arm64, fix it for perf */
pub const MAP_32BIT: i32 = 0;
