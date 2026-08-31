/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const MADV_DODUMP: i32 = 17;
pub const MADV_DOFORK: i32 = 11;
pub const MADV_DONTDUMP: i32 = 16;
pub const MADV_DONTFORK: i32 = 10;
pub const MADV_DONTNEED: i32 = 4;
pub const MADV_FREE: i32 = 8;
pub const MADV_HUGEPAGE: i32 = 14;
pub const MADV_HWPOISON: i32 = 100;
pub const MADV_MERGEABLE: i32 = 12;
pub const MADV_NOHUGEPAGE: i32 = 15;
pub const MADV_NORMAL: i32 = 0;
pub const MADV_RANDOM: i32 = 1;
pub const MADV_REMOVE: i32 = 9;
pub const MADV_SEQUENTIAL: i32 = 2;
pub const MADV_UNMERGEABLE: i32 = 13;
pub const MADV_WILLNEED: i32 = 3;
pub const MAP_ANONYMOUS: i32 = 0x0800;
pub const MAP_DENYWRITE: i32 = 0x2000;
pub const MAP_EXECUTABLE: i32 = 0x4000;
pub const MAP_FILE: i32 = 0;
pub const MAP_FIXED: i32 = 0x010;
pub const MAP_GROWSDOWN: i32 = 0x1000;
pub const MAP_HUGETLB: i32 = 0x80000;
pub const MAP_LOCKED: i32 = 0x8000;
pub const MAP_NONBLOCK: i32 = 0x20000;
pub const MAP_NORESERVE: i32 = 0x0400;
pub const MAP_POPULATE: i32 = 0x10000;
pub const MAP_STACK: i32 = 0x40000;
pub const PROT_EXEC: i32 = 0x04;
pub const PROT_GROWSDOWN: i32 = 0x01000000;
pub const PROT_GROWSUP: i32 = 0x02000000;
pub const PROT_NONE: i32 = 0x00;
pub const PROT_READ: i32 = 0x01;
pub const PROT_SEM: i32 = 0x10;
pub const PROT_WRITE: i32 = 0x02;

/* MADV_SOFT_OFFLINE is undefined on mips, fix it for perf */
pub const MADV_SOFT_OFFLINE: i32 = 101;

/* MAP_32BIT is undefined on mips, fix it for perf */
pub const MAP_32BIT: i32 = 0;

/* MAP_UNINITIALIZED is undefined on mips, fix it for perf */
pub const MAP_UNINITIALIZED: i32 = 0;
