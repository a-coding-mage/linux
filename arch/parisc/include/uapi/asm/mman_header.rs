/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const PROT_READ: i32 = 0x1; // page can be read
pub const PROT_WRITE: i32 = 0x2; // page can be written
pub const PROT_EXEC: i32 = 0x4; // page can be executed
pub const PROT_SEM: i32 = 0x8; // page may be used for atomic ops
pub const PROT_NONE: i32 = 0x0; // page can not be accessed
pub const PROT_GROWSDOWN: i32 = 0x01000000; // mprotect flag: extend change to start of growsdown vma
pub const PROT_GROWSUP: i32 = 0x02000000; // mprotect flag: extend change to end of growsup vma

// 0x01 - 0x03 are defined in linux/mman.h
pub const MAP_TYPE: i32 = 0x2b; // Mask for type of mapping, includes bits 0x08 and 0x20
pub const MAP_FIXED: i32 = 0x04; // Interpret addr exactly
pub const MAP_ANONYMOUS: i32 = 0x10; // don't use a file

pub const MAP_DENYWRITE: i32 = 0x0800; // ETXTBSY
pub const MAP_EXECUTABLE: i32 = 0x1000; // mark it as an executable
pub const MAP_LOCKED: i32 = 0x2000; // pages are locked
pub const MAP_NORESERVE: i32 = 0x4000; // don't check for reservations
pub const MAP_GROWSDOWN: i32 = 0x8000; // stack-like segment
pub const MAP_POPULATE: i32 = 0x10000; // populate (prefault) pagetables
pub const MAP_NONBLOCK: i32 = 0x20000; // do not block on IO
pub const MAP_STACK: i32 = 0x40000; // give out an address that is best suited for process/thread stacks
pub const MAP_HUGETLB: i32 = 0x80000; // create a huge page mapping
pub const MAP_FIXED_NOREPLACE: i32 = 0x100000; // MAP_FIXED which doesn't unmap underlying mapping
pub const MAP_UNINITIALIZED: i32 = 0; // uninitialized anonymous mmap

pub const MS_SYNC: i32 = 1; // synchronous memory sync
pub const MS_ASYNC: i32 = 2; // sync memory asynchronously
pub const MS_INVALIDATE: i32 = 4; // invalidate the caches

pub const MCL_CURRENT: i32 = 1; // lock all current mappings
pub const MCL_FUTURE: i32 = 2; // lock all future mappings
pub const MCL_ONFAULT: i32 = 4; // lock all pages that are faulted in

pub const MLOCK_ONFAULT: i32 = 0x01; // Lock pages in range after they are faulted in, do not prefault

pub const MADV_NORMAL: i32 = 0; // no further special treatment
pub const MADV_RANDOM: i32 = 1; // expect random page references
pub const MADV_SEQUENTIAL: i32 = 2; // expect sequential page references
pub const MADV_WILLNEED: i32 = 3; // will need these pages
pub const MADV_DONTNEED: i32 = 4; // don't need these pages

// common/generic parameters
pub const MADV_FREE: i32 = 8; // free pages only if memory pressure
pub const MADV_REMOVE: i32 = 9; // remove these pages & resources
pub const MADV_DONTFORK: i32 = 10; // don't inherit across fork
pub const MADV_DOFORK: i32 = 11; // do inherit across fork

pub const MADV_MERGEABLE: i32 = 12; // KSM may merge identical pages
pub const MADV_UNMERGEABLE: i32 = 13; // KSM may not merge identical pages

pub const MADV_HUGEPAGE: i32 = 14; // Worth backing with hugepages
pub const MADV_NOHUGEPAGE: i32 = 15; // Not worth backing with hugepages

pub const MADV_DONTDUMP: i32 = 16; // Explicity exclude from the core dump, overrides the coredump filter bits
pub const MADV_DODUMP: i32 = 17; // Clear the MADV_NODUMP flag

pub const MADV_WIPEONFORK: i32 = 18; // Zero memory on fork, child only
pub const MADV_KEEPONFORK: i32 = 19; // Undo MADV_WIPEONFORK

pub const MADV_COLD: i32 = 20; // deactivate these pages
pub const MADV_PAGEOUT: i32 = 21; // reclaim these pages

pub const MADV_POPULATE_READ: i32 = 22; // populate (prefault) page tables readable
pub const MADV_POPULATE_WRITE: i32 = 23; // populate (prefault) page tables writable

pub const MADV_DONTNEED_LOCKED: i32 = 24; // like DONTNEED, but drop locked pages too

pub const MADV_COLLAPSE: i32 = 25; // Synchronous hugepage collapse

pub const MADV_HWPOISON: i32 = 100; // poison a page for testing
pub const MADV_SOFT_OFFLINE: i32 = 101; // soft offline page for testing

pub const MADV_GUARD_INSTALL: i32 = 102; // fatal signal on access to range
pub const MADV_GUARD_REMOVE: i32 = 103; // unguard range

// compatibility flags
pub const MAP_FILE: i32 = 0;

pub const PKEY_DISABLE_ACCESS: i32 = 0x1;
pub const PKEY_DISABLE_WRITE: i32 = 0x2;
pub const PKEY_ACCESS_MASK: i32 = PKEY_DISABLE_ACCESS | PKEY_DISABLE_WRITE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
