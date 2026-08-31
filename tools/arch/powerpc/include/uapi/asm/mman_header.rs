// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const MAP_DENYWRITE: u32 = 0x0800;
pub const MAP_EXECUTABLE: u32 = 0x1000;
pub const MAP_GROWSDOWN: u32 = 0x0100;
pub const MAP_LOCKED: u32 = 0x80;
pub const MAP_NORESERVE: u32 = 0x40;

// C header dependency: <uapi/asm-generic/mman-common.h>

// MAP_32BIT is undefined on powerpc, fix it for perf
pub const MAP_32BIT: u32 = 0;
