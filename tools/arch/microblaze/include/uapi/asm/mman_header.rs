/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Rust translation of dependency intent: #include <uapi/asm-generic/mman.h> */

/* MAP_32BIT is undefined on microblaze, fix it for perf */
pub const MAP_32BIT: i32 = 0;
