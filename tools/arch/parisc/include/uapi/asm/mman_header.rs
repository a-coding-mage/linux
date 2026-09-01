/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const MADV_DODUMP: u32 = 17;
pub const MADV_DOFORK: u32 = 11;
pub const MADV_DONTDUMP: u32 = 16;
pub const MADV_DONTFORK: u32 = 10;
pub const MADV_DONTNEED: u32 = 4;
pub const MADV_FREE: u32 = 8;
pub const MADV_HUGEPAGE: u32 = 14;
pub const MADV_MERGEABLE: u32 = 12;
pub const MADV_NOHUGEPAGE: u32 = 15;
pub const MADV_NORMAL: u32 = 0;
pub const MADV_RANDOM: u32 = 1;
pub const MADV_REMOVE: u32 = 9;
pub const MADV_SEQUENTIAL: u32 = 2;
pub const MADV_UNMERGEABLE: u32 = 13;
pub const MADV_WILLNEED: u32 = 3;
pub const MAP_ANONYMOUS: u32 = 0x10;
pub const MAP_DENYWRITE: u32 = 0x0800;
pub const MAP_EXECUTABLE: u32 = 0x1000;
pub const MAP_FILE: u32 = 0;
pub const MAP_FIXED: u32 = 0x04;
pub const MAP_GROWSDOWN: u32 = 0x8000;
pub const MAP_HUGETLB: u32 = 0x80000;
pub const MAP_LOCKED: u32 = 0x2000;
pub const MAP_NONBLOCK: u32 = 0x20000;
pub const MAP_NORESERVE: u32 = 0x4000;
pub const MAP_POPULATE: u32 = 0x10000;
pub const MAP_STACK: u32 = 0x40000;
pub const PROT_EXEC: u32 = 0x4;
pub const PROT_GROWSDOWN: u32 = 0x01000000;
pub const PROT_GROWSUP: u32 = 0x02000000;
pub const PROT_NONE: u32 = 0x0;
pub const PROT_READ: u32 = 0x1;
pub const PROT_SEM: u32 = 0x8;
pub const PROT_WRITE: u32 = 0x2;
pub const MADV_HWPOISON: u32 = 100;
pub const MADV_SOFT_OFFLINE: u32 = 101;

/* MAP_32BIT is undefined on parisc, fix it for perf */
pub const MAP_32BIT: u32 = 0;
pub const MAP_UNINITIALIZED: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
