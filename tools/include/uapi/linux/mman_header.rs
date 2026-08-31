// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Depends on constants/types from asm/mman.h, asm-generic/hugetlb_encode.h,
// and linux/types.h.

pub const MREMAP_MAYMOVE: u32 = 1;
pub const MREMAP_FIXED: u32 = 2;
pub const MREMAP_DONTUNMAP: u32 = 4;

pub const OVERCOMMIT_GUESS: u32 = 0;
pub const OVERCOMMIT_ALWAYS: u32 = 1;
pub const OVERCOMMIT_NEVER: u32 = 2;

pub const MAP_SHARED: u32 = 0x01; // Share changes
pub const MAP_PRIVATE: u32 = 0x02; // Changes are private
pub const MAP_SHARED_VALIDATE: u32 = 0x03; // share + validate extension flags
pub const MAP_DROPPABLE: u32 = 0x08; // Zero memory under memory pressure.

/*
 * Huge page size encoding when MAP_HUGETLB is specified, and a huge page
 * size other than the default is desired.  See hugetlb_encode.h.
 * All known huge page size encodings are provided here.  It is the
 * responsibility of the application to know which sizes are supported on
 * the running system.  See mmap(2) man page for details.
 */
pub const MAP_HUGE_SHIFT: u32 = HUGETLB_FLAG_ENCODE_SHIFT;
pub const MAP_HUGE_MASK: u32 = HUGETLB_FLAG_ENCODE_MASK;

pub const MAP_HUGE_16KB: u32 = HUGETLB_FLAG_ENCODE_16KB;
pub const MAP_HUGE_64KB: u32 = HUGETLB_FLAG_ENCODE_64KB;
pub const MAP_HUGE_512KB: u32 = HUGETLB_FLAG_ENCODE_512KB;
pub const MAP_HUGE_1MB: u32 = HUGETLB_FLAG_ENCODE_1MB;
pub const MAP_HUGE_2MB: u32 = HUGETLB_FLAG_ENCODE_2MB;
pub const MAP_HUGE_8MB: u32 = HUGETLB_FLAG_ENCODE_8MB;
pub const MAP_HUGE_16MB: u32 = HUGETLB_FLAG_ENCODE_16MB;
pub const MAP_HUGE_32MB: u32 = HUGETLB_FLAG_ENCODE_32MB;
pub const MAP_HUGE_256MB: u32 = HUGETLB_FLAG_ENCODE_256MB;
pub const MAP_HUGE_512MB: u32 = HUGETLB_FLAG_ENCODE_512MB;
pub const MAP_HUGE_1GB: u32 = HUGETLB_FLAG_ENCODE_1GB;
pub const MAP_HUGE_2GB: u32 = HUGETLB_FLAG_ENCODE_2GB;
pub const MAP_HUGE_16GB: u32 = HUGETLB_FLAG_ENCODE_16GB;

#[repr(C)]
pub struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}

#[repr(C)]
pub struct cachestat {
    pub nr_cache: __u64,
    pub nr_dirty: __u64,
    pub nr_writeback: __u64,
    pub nr_evicted: __u64,
    pub nr_recently_evicted: __u64,
}
