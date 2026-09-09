/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2013 Red Hat Inc.
 *
 * Authors: Jérôme Glisse <jglisse@redhat.com>
 *
 * See Documentation/mm/hmm.rst for reasons and overview of what HMM is.
 */

// Dependency supplied by linux/mm.h.
#[allow(non_camel_case_types)]
pub enum mmu_interval_notifier {}

/*
 * On output:
 * 0             - The page is faultable and a future call with
 *                 HMM_PFN_REQ_FAULT could succeed.
 * HMM_PFN_VALID - the pfn field points to a valid PFN. This PFN is at
 *                 least readable. If dev_private_owner is !NULL then this could
 *                 point at a DEVICE_PRIVATE page.
 * HMM_PFN_WRITE - if the page memory can be written to (requires HMM_PFN_VALID)
 * HMM_PFN_ERROR - accessing the pfn is impossible and the device should
 *                 fail. ie poisoned memory, special pages, no vma, etc
 * HMM_PFN_P2PDMA - P2P page
 * HMM_PFN_P2PDMA_BUS - Bus mapped P2P transfer
 * HMM_PFN_DMA_MAPPED - Flag preserved on input-to-output transformation
 *                      to mark that page is already DMA mapped
 *
 * On input:
 * 0                 - Return the current state of the page, do not fault it.
 * HMM_PFN_REQ_FAULT - The output must have HMM_PFN_VALID or hmm_range_fault()
 *                     will fail
 * HMM_PFN_REQ_WRITE - The output must have HMM_PFN_WRITE or hmm_range_fault()
 *                     will fail. Must be combined with HMM_PFN_REQ_FAULT.
 */
#[repr(usize)]
#[allow(non_camel_case_types, dead_code)]
pub enum hmm_pfn_flags {
    /* Output fields and flags */
    HMM_PFN_VALID = 1usize << (usize::BITS - 1),
    HMM_PFN_WRITE = 1usize << (usize::BITS - 2),
    HMM_PFN_ERROR = 1usize << (usize::BITS - 3),
    /*
     * Sticky flags, carried from input to output,
     * don't forget to update HMM_PFN_INOUT_FLAGS
     */
    HMM_PFN_DMA_MAPPED = 1usize << (usize::BITS - 4),
    HMM_PFN_P2PDMA = 1usize << (usize::BITS - 5),
    HMM_PFN_P2PDMA_BUS = 1usize << (usize::BITS - 6),

    HMM_PFN_ORDER_SHIFT = usize::BITS - 11,

    /* Input flags */
    HMM_PFN_REQ_FAULT = 1usize << (usize::BITS - 1),
    HMM_PFN_REQ_WRITE = 1usize << (usize::BITS - 2),

    HMM_PFN_FLAGS = !( (1usize << (usize::BITS - 11)) - 1),
}

/* Dependency supplied by linux/mm.h. */
#[allow(non_camel_case_types)]
pub enum page {}

/*
 * hmm_pfn_to_page() - return struct page pointed to by a device entry
 *
 * This must be called under the caller 'user_lock' after a successful
 * mmu_interval_read_begin(). The caller must have tested for HMM_PFN_VALID
 * already.
 */
#[inline]
pub unsafe fn hmm_pfn_to_page(hmm_pfn: usize) -> *mut page {
    pfn_to_page(hmm_pfn & !((HMM_PFN_FLAGS as usize)))
}

/*
 * hmm_pfn_to_phys() - return physical address pointed to by a device entry
 */
#[inline]
pub unsafe fn hmm_pfn_to_phys(hmm_pfn: usize) -> usize {
    __pfn_to_phys(hmm_pfn & !((HMM_PFN_FLAGS as usize)))
}

/*
 * hmm_pfn_to_map_order() - return the CPU mapping size order
 *
 * This is optionally useful to optimize processing of the pfn result
 * array. It indicates that the page starts at the order aligned VA and is
 * 1<<order bytes long.  Every pfn within an high order page will have the
 * same pfn flags, both access protections and the map_order.  The caller must
 * be careful with edge cases as the start and end VA of the given page may
 * extend past the range used with hmm_range_fault().
 *
 * This must be called under the caller 'user_lock' after a successful
 * mmu_interval_read_begin(). The caller must have tested for HMM_PFN_VALID
 * already.
 */
#[inline]
pub fn hmm_pfn_to_map_order(hmm_pfn: usize) -> u32 {
    ((hmm_pfn >> (usize::BITS - 11)) & 0x1F) as u32
}

/*
 * struct hmm_range - track invalidation lock on virtual address range
 *
 * @notifier: a mmu_interval_notifier that includes the start/end
 * @notifier_seq: result of mmu_interval_read_begin()
 * @start: range virtual start address (inclusive)
 * @end: range virtual end address (exclusive)
 * @hmm_pfns: array of pfns (big enough for the range)
 * @default_flags: default flags for the range (write, read, ... see hmm doc)
 * @pfn_flags_mask: allows to mask pfn flags so that only default_flags matter
 * @dev_private_owner: owner of device private pages
 */
#[repr(C)]
pub struct hmm_range {
    pub notifier: *mut mmu_interval_notifier,
    pub notifier_seq: usize,
    pub start: usize,
    pub end: usize,
    pub hmm_pfns: *mut usize,
    pub default_flags: usize,
    pub pfn_flags_mask: usize,
    pub dev_private_owner: *mut core::ffi::c_void,
}

/*
 * Please see Documentation/mm/hmm.rst for how to use the range API.
 */
extern "C" {
    pub fn hmm_range_fault(range: *mut hmm_range) -> i32;
    pub fn hmm_range_fault_unlocked_timeout(range: *mut hmm_range, timeout: usize) -> i32;
}

/*
 * HMM_RANGE_DEFAULT_TIMEOUT - default timeout (ms) when waiting for a range
 *
 * When waiting for mmu notifiers we need some kind of time out otherwise we
 * could potentially wait for ever, 1000ms ie 1s sounds like a long time to
 * wait already.
 */
pub const HMM_RANGE_DEFAULT_TIMEOUT: usize = 1000;

// External functions supplied by linux/mm.h.
extern "C" {
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn __pfn_to_phys(pfn: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
