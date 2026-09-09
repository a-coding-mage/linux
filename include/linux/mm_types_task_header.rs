/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Here are the definitions of the MM data types that are embedded in
 * 'struct task_struct'.
 *
 * (These are defined separately to decouple sched.h from mm_types.h as much
 * as possible.)
 *
 * C dependencies supplied by other headers are intentionally referenced but
 * not implemented here.
 */

pub const ALLOC_SPLIT_PTLOCKS: bool = SPINLOCK_SIZE > BITS_PER_LONG / 8;

/*
 * When updating this, please also update struct resident_page_types[] in
 * kernel/fork.c
 */
#[repr(C)]
pub enum MmCounter {
    MM_FILEPAGES, /* Resident file mapping pages */
    MM_ANONPAGES, /* Resident anonymous pages */
    MM_SWAPENTS,  /* Anonymous swap entries */
    MM_SHMEMPAGES, /* Resident shared memory pages */
    NR_MM_COUNTERS,
}

pub struct page;

#[repr(C)]
pub struct page_frag {
    pub page: *mut page,
    /* C condition: (BITS_PER_LONG > 32) || (PAGE_SIZE >= 65536) */
    pub offset: u32,
    pub size: u32,
}

pub const PAGE_FRAG_CACHE_MAX_SIZE: usize = __ALIGN_MASK(32768, !PAGE_MASK);
/* C macro: get_order(PAGE_FRAG_CACHE_MAX_SIZE) */
pub const PAGE_FRAG_CACHE_MAX_ORDER: usize = get_order(PAGE_FRAG_CACHE_MAX_SIZE);

#[repr(C)]
pub struct page_frag_cache {
    /* encoded_page consists of the virtual address, pfmemalloc bit and
     * order of a page.
     */
    pub encoded_page: usize,

    /* we maintain a pagecount bias, so that we dont dirty cache line
     * containing page->_refcount every time we allocate a fragment.
     */
    /* C condition: (PAGE_SIZE < PAGE_FRAG_CACHE_MAX_SIZE) && (BITS_PER_LONG <= 32) */
    pub offset: u32,
    pub pagecnt_bias: u32,
}

/* Track pages that require TLB flushes */
#[repr(C)]
pub struct tlbflush_unmap_batch {
    /* C condition: CONFIG_ARCH_WANT_BATCHED_UNMAP_TLB_FLUSH */
    /*
     * The arch code makes the following promise: generic code can modify a
     * PTE, then call arch_tlbbatch_add_pending() (which internally provides
     * all needed barriers), then call arch_tlbbatch_flush(), and the entries
     * will be flushed on all CPUs by the time that arch_tlbbatch_flush()
     * returns.
     */
    pub arch: arch_tlbflush_unmap_batch,

    /* True if a flush is needed. */
    pub flush_required: bool,

    /*
     * If true then the PTE was dirty when unmapped. The entry must be
     * flushed before IO is initiated or a stale TLB entry potentially
     * allows an update without redirtying the page.
     */
    pub writable: bool,
}

#[repr(C)]
pub struct lazy_mmu_state {
    pub enable_count: u8,
    pub pause_count: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
