/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Stable page flag bits exported to user space
 */

pub const KPF_LOCKED: u32 = 0;
pub const KPF_ERROR: u32 = 1; /* Now unused */
pub const KPF_REFERENCED: u32 = 2;
pub const KPF_UPTODATE: u32 = 3;
pub const KPF_DIRTY: u32 = 4;
pub const KPF_LRU: u32 = 5;
pub const KPF_ACTIVE: u32 = 6;
pub const KPF_SLAB: u32 = 7;
pub const KPF_WRITEBACK: u32 = 8;
pub const KPF_RECLAIM: u32 = 9;
pub const KPF_BUDDY: u32 = 10;

/* 11-20: new additions in 2.6.31 */
pub const KPF_MMAP: u32 = 11;
pub const KPF_ANON: u32 = 12;
pub const KPF_SWAPCACHE: u32 = 13;
pub const KPF_SWAPBACKED: u32 = 14;
pub const KPF_COMPOUND_HEAD: u32 = 15;
pub const KPF_COMPOUND_TAIL: u32 = 16;
pub const KPF_HUGE: u32 = 17;
pub const KPF_UNEVICTABLE: u32 = 18;
pub const KPF_HWPOISON: u32 = 19;
pub const KPF_NOPAGE: u32 = 20;

pub const KPF_KSM: u32 = 21;
pub const KPF_THP: u32 = 22;
pub const KPF_OFFLINE: u32 = 23;
pub const KPF_ZERO_PAGE: u32 = 24;
pub const KPF_IDLE: u32 = 25;
pub const KPF_PGTABLE: u32 = 26;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
