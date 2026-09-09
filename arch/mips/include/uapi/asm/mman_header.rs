/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1995, 1999, 2002 by Ralf Baechle
 */

/*
 * Protections are chosen from these bits, OR'd together.  The
 * implementation does not necessarily support PROT_EXEC or PROT_WRITE
 * without PROT_READ.  The only guarantees are that no writing will be
 * allowed without PROT_WRITE and no access will be allowed for PROT_NONE.
 */
pub const PROT_NONE: u32 = 0x00; // page can not be accessed
pub const PROT_READ: u32 = 0x01; // page can be read
pub const PROT_WRITE: u32 = 0x02; // page can be written
pub const PROT_EXEC: u32 = 0x04; // page can be executed
// 0x08 is reserved for PROT_EXEC_NOFLUSH.
pub const PROT_SEM: u32 = 0x10; // page may be used for atomic ops
pub const PROT_GROWSDOWN: u32 = 0x01000000; // mprotect flag: extend change to start of growsdown vma
pub const PROT_GROWSUP: u32 = 0x02000000; // mprotect flag: extend change to end of growsup vma

/* Flags for mmap */
pub const MAP_TYPE: u32 = 0x00f; // Mask for type of mapping
pub const MAP_FIXED: u32 = 0x010; // Interpret addr exactly

// Not used by Linux, but here to make sure we don't clash with ABI defines.
pub const MAP_RENAME: u32 = 0x020; // Assign page to file
pub const MAP_AUTOGROW: u32 = 0x040; // File may grow by writing
pub const MAP_LOCAL: u32 = 0x080; // Copy on fork/sproc
pub const MAP_AUTORSRV: u32 = 0x100; // Logical swap reserved on demand

/* These are linux-specific */
pub const MAP_NORESERVE: u32 = 0x0400; // don't check for reservations
pub const MAP_ANONYMOUS: u32 = 0x0800; // don't use a file
pub const MAP_GROWSDOWN: u32 = 0x1000; // stack-like segment
pub const MAP_DENYWRITE: u32 = 0x2000; // ETXTBSY
pub const MAP_EXECUTABLE: u32 = 0x4000; // mark it as an executable
pub const MAP_LOCKED: u32 = 0x8000; // pages are locked
pub const MAP_POPULATE: u32 = 0x10000; // populate (prefault) pagetables
pub const MAP_NONBLOCK: u32 = 0x20000; // do not block on IO
pub const MAP_STACK: u32 = 0x40000; // give out an address that is best suited for process/thread stacks
pub const MAP_HUGETLB: u32 = 0x80000; // create a huge page mapping
pub const MAP_FIXED_NOREPLACE: u32 = 0x100000; // MAP_FIXED which doesn't unmap underlying mapping

/* Flags for msync */
pub const MS_ASYNC: u32 = 0x0001; // sync memory asynchronously
pub const MS_INVALIDATE: u32 = 0x0002; // invalidate mappings & caches
pub const MS_SYNC: u32 = 0x0004; // synchronous memory sync

/* Flags for mlockall */
pub const MCL_CURRENT: u32 = 1; // lock all current mappings
pub const MCL_FUTURE: u32 = 2; // lock all future mappings
pub const MCL_ONFAULT: u32 = 4; // lock all pages that are faulted in

/* Flags for mlock */
pub const MLOCK_ONFAULT: u32 = 0x01; // Lock pages in range after they are faulted in, do not prefault

pub const MADV_NORMAL: u32 = 0; // no further special treatment
pub const MADV_RANDOM: u32 = 1; // expect random page references
pub const MADV_SEQUENTIAL: u32 = 2; // expect sequential page references
pub const MADV_WILLNEED: u32 = 3; // will need these pages
pub const MADV_DONTNEED: u32 = 4; // don't need these pages

/* common parameters: try to keep these consistent across architectures */
pub const MADV_FREE: u32 = 8; // free pages only if memory pressure
pub const MADV_REMOVE: u32 = 9; // remove these pages & resources
pub const MADV_DONTFORK: u32 = 10; // don't inherit across fork
pub const MADV_DOFORK: u32 = 11; // do inherit across fork
pub const MADV_MERGEABLE: u32 = 12; // KSM may merge identical pages
pub const MADV_UNMERGEABLE: u32 = 13; // KSM may not merge identical pages
pub const MADV_HWPOISON: u32 = 100; // poison a page for testing
pub const MADV_HUGEPAGE: u32 = 14; // Worth backing with hugepages
pub const MADV_NOHUGEPAGE: u32 = 15; // Not worth backing with hugepages
pub const MADV_DONTDUMP: u32 = 16; // Explicitly exclude from core dump, overrides the coredump filter bits
pub const MADV_DODUMP: u32 = 17; // Clear the MADV_NODUMP flag
pub const MADV_WIPEONFORK: u32 = 18; // Zero memory on fork, child only
pub const MADV_KEEPONFORK: u32 = 19; // Undo MADV_WIPEONFORK
pub const MADV_COLD: u32 = 20; // deactivate these pages
pub const MADV_PAGEOUT: u32 = 21; // reclaim these pages
pub const MADV_POPULATE_READ: u32 = 22; // populate (prefault) page tables readable
pub const MADV_POPULATE_WRITE: u32 = 23; // populate (prefault) page tables writable
pub const MADV_DONTNEED_LOCKED: u32 = 24; // like DONTNEED, but drop locked pages too
pub const MADV_COLLAPSE: u32 = 25; // Synchronous hugepage collapse
pub const MADV_GUARD_INSTALL: u32 = 102; // fatal signal on access to range
pub const MADV_GUARD_REMOVE: u32 = 103; // unguard range

/* compatibility flags */
pub const MAP_FILE: u32 = 0;
pub const PKEY_DISABLE_ACCESS: u32 = 0x1;
pub const PKEY_DISABLE_WRITE: u32 = 0x2;
pub const PKEY_ACCESS_MASK: u32 = PKEY_DISABLE_ACCESS | PKEY_DISABLE_WRITE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
