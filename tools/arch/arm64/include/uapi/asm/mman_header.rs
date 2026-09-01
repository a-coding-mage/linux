/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Rust translation of <uapi/asm-generic/mman.h> dependency. */

/* MAP_32BIT is undefined on arm64, fix it for perf */
pub const MAP_32BIT: i32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
